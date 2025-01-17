
use time::Duration;
use std::collections::HashMap;
use serde_json;

use std::path::Path;
use std::mem;
use std::fs::File;
use std::io::prelude::*;

//type Plot = HashMap<String, Vec<(u64, f64)>>;

// exact mirror of time::Duration
#[derive(Clone, Serialize, Deserialize, Eq, PartialEq, Debug)]
pub struct Moment {
    secs: i64,
    nanos: i32,
}
impl Moment {
    pub fn from_dur(t: Duration) -> Moment {
        unsafe {
            mem::transmute(t)
        }
    }

    pub fn into_dur(self) -> Duration {
        unsafe {
            mem::transmute(self)
        }
    }
}

pub trait Named {
    fn name(&self) -> &'static str;
}

// code for demographics
#[derive(Clone, Serialize, Deserialize, Eq, PartialEq, Debug)]
pub enum Age {
    UnderThirteen,
    ThirteenToEighteen,
    EighteenToThirty,
    ThirtyToFifty,
    FiftyAndOlder
}
const AGES: &[Age] = &[
    Age::UnderThirteen,
    Age::ThirteenToEighteen,
    Age::EighteenToThirty,
    Age::ThirtyToFifty,
    Age::FiftyAndOlder
];
impl Named for Age {
    fn name(&self) -> &'static str {
        match self {
            &Age::UnderThirteen => "UnderThirteen",
            &Age::ThirteenToEighteen => "ThirteenToEighteen",
            &Age::EighteenToThirty => "EighteenToThirty",
            &Age::ThirtyToFifty => "ThirtyToFifty",
            &Age::FiftyAndOlder => "FiftyAndOlder"
        }
    }
}


#[derive(Clone, Serialize, Deserialize, Eq, PartialEq, Debug)]
pub enum Gender {
    Female,
    Male,
    Other
}
const GENDERS: &[Gender] = &[
    Gender::Female,
    Gender::Male,
    Gender::Other
];
impl Named for Gender {
    fn name(&self) -> &'static str {
        match self {
            &Gender::Female => "Female",
            &Gender::Male => "Male",
            &Gender::Other => "Other"
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Eq, PartialEq, Debug)]
pub enum Continent {
    NorthAmerica,
    SouthAmerica,
    Europe,
    Africa,
    Asia,
    Austrilia,
    Antarctica
}
const CONTINENTS: &[Continent] = &[
    Continent::NorthAmerica,
    Continent::SouthAmerica,
    Continent::Europe,
    Continent::Africa,
    Continent::Asia,
    Continent::Austrilia,
    Continent::Antarctica
];
impl Named for Continent {
    fn name(&self) -> &'static str {
        match self {
            &Continent::NorthAmerica => "NorthAmerica",
            &Continent::SouthAmerica => "SouthAmerica",
            &Continent::Europe => "Europe",
            &Continent::Africa => "Africa",
            &Continent::Asia => "Asia",
            &Continent::Austrilia => "Australia",
            &Continent::Antarctica => "Antarctica"
        }
    }
}

// struct for a purchase
#[derive(Clone, Serialize, Deserialize, Eq, PartialEq, Debug)]
pub struct Purchase {
    pub age: Age,
    pub gender: Gender,
    pub continent: Continent,
    pub time: Moment
}
impl Purchase {
    pub fn get_dem_by_name(&self, name: &str) -> &Named {
        match name {
            "Age" => &self.age,
            "Gender" => &self.gender,
            "Continent" => &self.continent,
            _ => panic!("invalid name")
        }
    }
}

pub struct Database {
    points: Vec<Purchase>,
    path: String
}

// xy point struct
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct XY {
    x: f64,
    y: f64
}

// the json format for POSTing a buy
#[derive(Serialize, Deserialize, Debug)]
pub struct BuyPost {
    time: u64,
    continent: String,
    gender: String,
    age: String
}
impl BuyPost {
    pub fn into_purchase(self) -> Purchase {
        Purchase {
            age: AGES.iter().find(|a| a.name() == &self.age[..]).unwrap().clone(),
            gender: GENDERS.iter().find(|g| g.name() == &self.gender[..]).unwrap().clone(),
            continent: CONTINENTS.iter().find(|c| c.name() == &self.continent[..]).unwrap().clone(),
            time: Moment::from_dur(Duration::milliseconds(self.time as i64))
        }
    }
}

pub type Response = HashMap<String, HashMap<String, Vec<XY>>>;

// the database struct
impl Database {
    pub fn new(path: String) -> Database {
        match File::open(&path) {
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents);

                match serde_json::from_str::<Vec<Purchase>>(&contents[..]) {
                    Ok(points) => {
                        println!("read database");
                        Database {
                            points,
                            path
                        }
                    },
                    Err(err) => {
                        println!("failed to parse database json: {:?}", err);
                        Database {
                            points: Vec::new(),
                            path
                        }
                    }
                }
            },
            Err(_) => {
                println!("database not found");
                Database {
                    points: Vec::new(),
                    path
                }
            }
        }
    }

    pub fn save(&self) {
        let mut file = File::create(&self.path).expect("save file creation error");
        let str = serde_json::to_string(&self.points).unwrap();
        file.write_all(str.as_bytes());
    }

    pub fn close(self) {
        mem::drop(self)
    }

    pub fn form_response(&self) -> Response {
        let mut outer: HashMap<String, HashMap<String, Vec<XY>>> = HashMap::new();

        // build ages
        {
            let mut ages: HashMap<String, Vec<XY>> = HashMap::new();
            for age in AGES {
                let mut histogram = HashMap::<i64, u64>::new();
                for point in self.points.iter() {
                    if &point.age == age {
                        let day = point.time.clone().into_dur().num_days();

                        let insert = {
                            if let Some(mut i) = histogram.get_mut(&day) {
                                *i += 1;
                                false
                            } else {
                                true
                            }
                        };
                        if insert {
                            histogram.insert(day, 1);
                        }
                    }
                }
                let mut points: Vec<XY> = histogram.iter()
                    .map(|pair| XY {
                        x: *pair.0 as f64,
                        y: *pair.1 as f64
                    })
                    .collect();
                points.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());
                ages.insert(age.name().to_owned(), points);
            }
            outer.insert("Age".to_owned(), ages);
        }

        // build genders
        {
            let mut genders: HashMap<String, Vec<XY>> = HashMap::new();
            for gender in GENDERS {
                let mut histogram = HashMap::<i64, u64>::new();
                for point in self.points.iter() {
                    if &point.gender == gender {
                        let day = point.time.clone().into_dur().num_days();

                        let insert = {
                            if let Some(mut i) = histogram.get_mut(&day) {
                                *i += 1;
                                false
                            } else {
                                true
                            }
                        };
                        if insert {
                            histogram.insert(day, 1);
                        }
                    }
                }
                let mut points: Vec<XY> = histogram.iter()
                    .map(|pair| XY {
                        x: *pair.0 as f64,
                        y: *pair.1 as f64
                    })
                    .collect();
                points.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());
                genders.insert(gender.name().to_owned(), points);
            }
            outer.insert("Gender".to_owned(), genders);
        }

        // build continents
        {
            let mut continents: HashMap<String, Vec<XY>> = HashMap::new();
            for continent in CONTINENTS {
                let mut histogram = HashMap::<i64, u64>::new();
                for point in self.points.iter() {
                    if &point.continent == continent {
                        let day = point.time.clone().into_dur().num_days();

                        let insert = {
                            if let Some(mut i) = histogram.get_mut(&day) {
                                *i += 1;
                                false
                            } else {
                                true
                            }
                        };
                        if insert {
                            histogram.insert(day, 1);
                        }
                    }
                }
                let mut points: Vec<XY> = histogram.iter()
                    .map(|pair| XY {
                        x: *pair.0 as f64,
                        y: *pair.1 as f64
                    })
                    .collect();
                points.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());
                continents.insert(continent.name().to_owned(), points);
            }
            outer.insert("Continent".to_owned(), continents);
        }

        outer
    }

    pub fn form_response_json(&self) -> String {
        let response = self.form_response();
        serde_json::to_string(&response).unwrap()
    }

    pub fn add_point(&mut self, point: Purchase) {
        self.points.push(point);
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string(&self.points).unwrap()
    }

    pub fn len(&self) -> usize {
        self.points.len()
    }

    pub fn clear(&mut self) {
        self.points.clear()
    }
}

// save on drop
impl Drop for Database {
    fn drop(&mut self) {
        self.save()
    }
}
