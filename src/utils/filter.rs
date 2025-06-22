use serde::Serialize;

#[derive(Serialize, Default)]
pub struct FilterRequest {
    country: Option<()>,
    season: Option<()>,
    sort: u8,
    studio: Option<()>,
    // #[serde(rename = "age_ratings")]
    age_ratings: Vec<()>,
    category_id: Option<u8>,
    end_year: Option<()>,
    episode_duration_from: Option<()>,
    episode_duration_to: Option<()>,
    episodes_from: Option<()>,
    episodes_to: Option<()>,
    genres: Vec<()>,
    is_genres_exclude_mode_enabled: bool,
    profile_list_exclusions: Vec<()>,
    start_year: Option<u32>,
    status_id: Option<u8>,
    types: Vec<u32>,
}

pub struct FilterRequestBuilder {
    country: Option<()>,
    season: Option<()>,
    sort: u8,
    studio: Option<()>,
    age_ratings: Vec<()>,
    category_id: Option<u8>,
    end_year: Option<()>,
    episode_duration_from: Option<()>,
    episode_duration_to: Option<()>,
    episodes_from: Option<()>,
    episodes_to: Option<()>,
    genres: Vec<()>,
    is_genres_exclude_mode_enabled: bool,
    profile_list_exclusions: Vec<()>,
    start_year: Option<u32>,
    status_id: Option<u8>,
    types: Vec<u32>,
}

impl Default for FilterRequestBuilder {
    fn default() -> Self {
        Self {
            country: None,
            season: None,
            sort: 0,
            studio: None,
            age_ratings: Vec::new(),
            category_id: None, // 0 - Не известно, 1 - "Сериал", 2 - "фильмы", 3 - "ova", 4 - "дорама"
            end_year: None,
            episode_duration_from: None,
            episode_duration_to: None,
            episodes_from: None,
            episodes_to: None,
            genres: Vec::new(),
            is_genres_exclude_mode_enabled: false,
            profile_list_exclusions: Vec::new(),
            start_year: None,
            status_id: None, // 0 - "Не известно", 1 - "завершено", 2 - "онгоинги", 3 - "Анонс"
            types: Vec::new(),
        }
    }
    
}

impl FilterRequestBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn country(mut self, country: Option<()>) -> Self {
        self.country = country;
        self
    }

    pub fn season(mut self, season: Option<()>) -> Self {
        self.season = season;
        self
    }

    pub fn sort(mut self, sort: u8) -> Self {
        self.sort = sort;
        self
    }

    pub fn studio(mut self, studio: Option<()>) -> Self {
        self.studio = studio;
        self
    }

    pub fn age_ratings(mut self, age_ratings: Vec<()>) -> Self {
        self.age_ratings = age_ratings;
        self
    }

    pub fn category_id(mut self, category_id: Option<u8>) -> Self {
        self.category_id = category_id;
        self
    }
    pub fn end_year(mut self, end_year: Option<()>) -> Self {
        self.end_year = end_year;
        self
    }
    pub fn episode_duration_from(mut self, episode_duration_from: Option<()>) -> Self {
        self.episode_duration_from = episode_duration_from;
        self
    }
    pub fn episode_duration_to(mut self, episode_duration_to: Option<()>) -> Self {
        self.episode_duration_to = episode_duration_to;
        self
    }
    pub fn episodes_from(mut self, episodes_from: Option<()>) -> Self {
        self.episodes_from = episodes_from;
        self
    }
    pub fn episodes_to(mut self, episodes_to: Option<()>) -> Self {
        self.episodes_to = episodes_to;
        self
    }
    pub fn genres(mut self, genres: Vec<()>) -> Self {
        self.genres = genres;
        self
    }
    pub fn is_genres_exclude_mode_enabled(mut self, is_genres_exclude_mode_enabled: bool) -> Self {
        self.is_genres_exclude_mode_enabled = is_genres_exclude_mode_enabled;
        self
    }
    pub fn profile_list_exclusions(mut self, profile_list_exclusions: Vec<()>) -> Self {
        self.profile_list_exclusions = profile_list_exclusions;
        self
    }
    pub fn start_year(mut self, start_year: Option<u32>) -> Self {
        self.start_year = start_year;
        self
    }
    pub fn status_id(mut self, status_id: Option<u8>) -> Self {
        self.status_id = status_id;
        self
    }
    pub fn types(mut self, types: Vec<u32>) -> Self {
        self.types = types;
        self
    }
    pub fn build(self) -> FilterRequest {
        FilterRequest {
            country: self.country,
            season: self.season,
            sort: self.sort,
            studio: self.studio,
            age_ratings: self.age_ratings,
            category_id: self.category_id,
            end_year: self.end_year,
            episode_duration_from: self.episode_duration_from,
            episode_duration_to: self.episode_duration_to,
            episodes_from: self.episodes_from,
            episodes_to: self.episodes_to,
            genres: self.genres,
            is_genres_exclude_mode_enabled: self.is_genres_exclude_mode_enabled,
            profile_list_exclusions: self.profile_list_exclusions,
            start_year: self.start_year,
            status_id: self.status_id,
            types: self.types,
        }
    }
}