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
pub struct Value {
    index: usize,
    intensity: Intensity
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

//Trajectory is an ordered sequence of coordinates in possibility space
//Possibility space is the cross product of the asssemblage's relations
type Coordinate = Vec<Value>;
type Trajectory = Vec<Coordinate>;

//Components and relations should be thought of as a set. All components and relations should be unique.
//For memory purposes we will use vectors, so we can list relations as the indices of the vector of components.
#[derive(Debug, Default)]
pub struct Assemblage {
    components: Vec<Box<Assemblage>>,
    relations: Option<Vec<ExtrinsicRelation>>,
    trajectory: Option<Trajectory>
}

pub fn initial_relations() -> Option<Vec<ExtrinsicRelation>> {
    let r = ExtrinsicRelation {
        source: 0, target: 0, intensity: 0.0, label: Some("self".to_string())
    };
    Some(vec![r])
}

pub fn initial_trajectory() -> Option<Trajectory> {
    let v = Value {
        index: 0, intensity: 0.0
    };
    Some(vec![vec![v]])
}

impl Assemblage {

    pub fn new() -> Assemblage {
        //Look up Deleuze empty set (Lacan & Deleuze)
        //There is always one element in the empty set, the empty set itself.
        //This is how we can have a self relation
        let a = Assemblage {
            components: Vec::new(),
            .. Default::default()
        };
        
        Assemblage {
            components: vec![Box::new(a)],
            relations: initial_relations(),
            trajectory: initial_trajectory()
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
        0.0
    }

    pub fn codification(self) -> f64 {
        0.0
    }

    pub fn push_component(mut self, a: Assemblage) -> Self {
        self.components.push(Box::new(a));
        self
    }

    pub fn push_relation(mut self, r: ExtrinsicRelation) -> Self {
        self.relations = match self.relations {
            Some(mut rs) => {
                rs.push(r);
                Some(rs)
            }, None => {
                None
            }
        };
        self
    }

    pub fn push_value(mut self, index: usize, v: Value) -> Self {
        self.trajectory = match self.trajectory {
            Some(mut t) => {
                t[index].push(v);
                Some(t)
            }, None => {
                None
            }
        };
        self
    }

    pub fn push_coordinate(mut self, c: Coordinate) -> Self {
        self.trajectory = match self.trajectory {
            Some(mut t) => {
                t.push(c);
                Some(t)
            }, None => {
                None
            }
        };
        self
    }

}

impl fmt::Display for Assemblage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

pub fn add(a: Assemblage, b: Assemblage) -> Assemblage {    
    Assemblage::new()
        .push_component(a)
        .push_component(b)
        .push_relation(ExtrinsicRelation {
            source: 0,
            target: 1,
            intensity: 0.0,
            label: None
        })
        .push_relation(ExtrinsicRelation {
            source: 0,
            target: 2,
            intensity: 0.0,
            label: None
        })
        .push_relation(ExtrinsicRelation {
            source: 1,
            target: 2,
            intensity: 0.0,
            label: None
        })
        .push_value(0, Value {
            index: 1,
            intensity: 0.0
        })
        .push_value(0, Value {
            index: 2,
            intensity: 0.0
        })
        .push_value(0, Value {
            index: 3,
            intensity: 0.0
        })
}
