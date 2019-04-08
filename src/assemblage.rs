use std::fmt;

type Intensity = f64;

#[derive(Debug)]
pub struct ExtrinsicRelation {
    source: usize,
    target: usize,
    intensity: Intensity,
    label: Option<String>
}

impl fmt::Display for ExtrinsicRelation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

#[derive(Debug)]
pub struct Coordinate {
    index: usize,
    intensity: Intensity
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

type Trajectory = Vec<Coordinate>;
type PossibilitySpace = Vec<Vec<Coordinate>>;

//Components and relations should be thought of as a set. All components and relations should be unique.
//For memory purposes we will use vectors, so we can list relations as the indices of the vector of components.
#[derive(Debug, Default)]
pub struct Assemblage {
    components: Vec<Box<Assemblage>>,
    relations: Option<Vec<ExtrinsicRelation>>,
    trajectory: Option<Trajectory>
}

impl Assemblage {

    //identity
    pub fn new() -> Assemblage {
        Assemblage {
            components: Vec::new(),
            ..Default::default()
        }
    }

    pub fn components(mut self, components: Vec<Box<Assemblage>>) -> Self {
        self.components = components;
        self
    }

    pub fn relations(mut self, relations: Vec<ExtrinsicRelation>) -> Self {
        self.relations = Some(relations);
        self
    }

    pub fn trajectory(mut self, trajectory: Trajectory) -> Self {
        self.trajectory = Some(trajectory);
        self
    }

    pub fn territorialization(self) -> f64 {
        42.0
    }

    pub fn codification(self) -> f64 {
        42.0
    }

}

impl fmt::Display for Assemblage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

pub fn add(a: Assemblage, b: Assemblage) -> Assemblage {
    Assemblage::new()
        .components(vec![Box::new(a), Box::new(b)])
}
