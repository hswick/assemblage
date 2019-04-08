use std::fmt;

#[derive(Debug, Clone)]
pub struct Assemblage {
    name: String,
    relation: Option<(Box<Assemblage>, Box<Assemblage>)>,
    intensity: f64,
    log: Vec<f64>
}

impl Assemblage {

    pub fn new() -> Assemblage {
        Assemblage {
            name: "Null".to_string(),
            intensity: 0.0,
            log: vec![0.0],
            relation: None
        }
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn intensity(mut self, intensity: f64) -> Self {
        self.intensity = intensity;
        self.log = vec![intensity];
        self
    }

    pub fn relation(mut self, relation: (Assemblage, Assemblage)) -> Self {
        let (a, b) = relation;
        self.relation = Some((Box::new(a), Box::new(b)));
        self
    }

    // Difference in degree
    pub fn update(mut self, intensity: f64) -> Self {
        match self.relation {
            Some(_) => {
                self.intensity = intensity;
                self.log.push(intensity);
                self
            },
            None => { self }
        }
    }

}

impl fmt::Display for Assemblage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

pub fn null() -> Assemblage {
    Assemblage::new()
}

// Define the relation between two assemblages.
pub fn and(a: Assemblage, b: Assemblage, name: &str, intensity: f64) -> Assemblage {
    Assemblage::new()
        .name(name)
        .intensity(intensity)
        .relation((a, b))
}
