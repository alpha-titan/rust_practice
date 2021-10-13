
pub trait FourWheeler  {
    fn create (name:String, year: i32, model:String, top_speed:f64, total_gear: i16, driver:Driver) -> Self;
    
    fn move_left(&self);
    fn move_right(&self);
    fn move_forward(&self);
    fn move_backwards(&self);
    fn change_gears(&self, gear:u8);
    // fn get_weight (&self) -> f32;
    fn stop (&self);
    // fn curr_speed (&self) -> f64; 
}

pub trait Engine {
    fn start (&self);
    fn drive(&self);
}

#[derive(Debug)]
pub struct Driver {
    pub name:String,
    pub age: i32,
    pub contract_period: i32

}

#[derive(Debug)]
pub struct FormulaOneCar {
    pub name:String,
    pub model:String,
    pub year:i32,
    pub top_speed:f64,
    pub total_gear:i16,
    pub driver: Driver
}

impl FormulaOneCar {
    pub fn toString(&self) {
        println!("{:?}", &self);
    }

}

impl FourWheeler for FormulaOneCar {
    fn create(name:String,year: i32, model: String, top_speed:f64, total_gear:i16, driver:Driver) -> FormulaOneCar {
        FormulaOneCar{name, year, model, top_speed, total_gear, driver}
    }
    fn move_left(&self) {
        println!("Turning Left");
    }
    fn move_right(&self) {
        println!("Truning Right");
    }
    fn move_forward(&self) {
        println!("driving straight");
    }
    fn move_backwards(&self) {
        println!("taking reverse .......")
    }
    fn change_gears(&self, gear:u8) {
        println!("Shifting gear to {}", gear);
    }
    fn stop(&self) {
        println!("Stopping.........");
    }

}

impl Engine for FormulaOneCar {
    fn start(&self) {
        println!("Engine Started");
        println!("Vroom Vroom !!!!");
    }
    fn drive(&self) {
        println!("Driving .....")
    }
}


