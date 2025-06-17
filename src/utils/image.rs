use blake3::Hasher;
use gdk::{Paintable, Texture};
use glib::{Bytes, clone};
use gtk::Image;
use hex::encode as hex_encode;
use lru::LruCache;
use once_cell::sync::Lazy;
use std::fs;
use std::io::Write;
use std::num::NonZeroUsize;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tokio::sync::Semaphore;

use crate::api_client::AnixartClient;

static REQUEST_LIMITER: once_cell::sync::Lazy<Arc<Semaphore>> =
    once_cell::sync::Lazy::new(|| Arc::new(Semaphore::new(3)));

static IMAGE_CACHE: Lazy<Mutex<LruCache<String, Texture>>> = Lazy::new(|| {
    let cache_size = NonZeroUsize::new(100).unwrap(); // 100 изображений
    Mutex::new(LruCache::new(cache_size))
});

pub async fn download_image_async(
    url: &str,
) -> Result<gtk::gdk::Texture, Box<dyn std::error::Error + Send + Sync>> {
    let _permit = REQUEST_LIMITER.acquire().await?; // Ограничение количества одновременных запросов
    if has_cached_image(url) {
        if let Some(texture) = IMAGE_CACHE.lock().unwrap().get(url).cloned() {
            return Ok(texture);
        }

        // Проверка дискового кэша
        if let Some(texture) = load_cached_image(url) {
            // Сохраняем в кэш памяти для быстрого доступа
            IMAGE_CACHE
                .lock()
                .unwrap()
                .put(url.to_string(), texture.clone());
            return Ok(texture);
        }
    }

    let client = AnixartClient::global();

    let response = client.client.get(url).send().await?;
    let bytes = response.bytes().await?;

    save_to_cache(url, &bytes);

    let texture = Texture::from_bytes(&Bytes::from(&bytes[..])).expect("Failed to create texture");

    IMAGE_CACHE
        .lock()
        .unwrap()
        .put(url.to_string(), texture.clone());

    Ok(texture)
}

pub fn load_image(image: &Image, url: &str) {
    let url = url.to_string();
    let (sender, receiver) = async_channel::bounded::<
        Result<gtk::gdk::Texture, Box<dyn std::error::Error + Send + Sync>>,
    >(3);
    tokio::spawn(clone!(
        #[strong]
        sender,
        async move {
            let _ = sender.send(download_image_async(&url).await).await;
        }
    ));

    let image_clone = image.clone();
    glib::spawn_future_local(async move {
        while let Ok(response) = receiver.recv().await {
            let image_clone = image_clone.clone();
            match response {
                Ok(texture) => {
                    image_clone.set_paintable(Some(&texture));
                }
                Err(_e) => {
                    image_clone.set_paintable(Some(&create_placeholder()));
                }
            };
        }
    });
}

fn create_placeholder() -> Paintable {
    let width = 100;
    let height = 100;

    Paintable::new_empty(width, height)
}

fn hash_url(url: &str) -> String {
    let mut hasher = Hasher::new();
    hasher.update(url.as_bytes());
    let hash = hasher.finalize();
    hex_encode(hash.as_bytes())
}

// Получение пути к кэш-директории
fn cache_dir() -> PathBuf {
    let mut dir = glib::user_cache_dir();
    dir.push("anixart-app");
    dir.push("CacheStorage");
    dir
}

// Проверка существования кэшированного изображения
fn has_cached_image(url: &str) -> bool {
    let hash = hash_url(url);
    let path = cache_dir().join(&hash);
    path.exists()
}

// Загрузка изображения из кэша
fn load_cached_image(url: &str) -> Option<gdk::Texture> {
    let hash = hash_url(url);
    let path = cache_dir().join(&hash);

    match fs::read(&path) {
        Ok(bytes) => Texture::from_bytes(&glib::Bytes::from(&bytes))
            .map_err(|e| eprintln!("Cache load error: {}", e))
            .ok(),
        Err(e) => {
            eprintln!("Cache read error: {} - {}", path.display(), e);
            None
        }
    }
}

// Сохранение изображения в кэш
fn save_to_cache(url: &str, bytes: &[u8]) {
    let cache_dir = cache_dir();
    if !cache_dir.exists() {
        if let Err(e) = fs::create_dir_all(&cache_dir) {
            eprintln!("Failed to create cache dir: {}", e);
            return;
        }
    }

    let hash = hash_url(url);
    let path = cache_dir.join(&hash);

    match fs::File::create(&path).and_then(|mut file| file.write_all(bytes)) {
        Ok(_) => println!("Cached image: {}", hash),
        Err(e) => eprintln!("Cache save error: {} - {}", path.display(), e),
    }
}

pub fn _clear_image_cache() {
    // Очистка кэша в памяти
    IMAGE_CACHE.lock().unwrap().clear();

    // Очистка дискового кэша
    let cache_dir = cache_dir();
    if cache_dir.exists() {
        if let Err(e) = fs::remove_dir_all(&cache_dir) {
            eprintln!("Failed to clear cache: {}", e);
        } else {
            println!("Image cache cleared");
        }
    }
}
