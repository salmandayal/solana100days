//Rust doesn't support inheritance but only support interface by only defining "Trait"
pub fn create_struct(){
    let car_type = String::from("Sedan");
    let model = String::from("Honda Reborn");
    let my_car = Car{car_type,model,hp:120};
    my_car.show_hp();
    my_car.accelerate();
    println!("My car is electric :{}",my_car.is_electric());
}

struct Car {
    car_type:String, hp:i32,model:String
}

impl Car {
    fn accelerate(&self){
        println!("Vroom Vroom");
    }
    fn show_hp(&self){
        println!("Horses:{}",self.hp);
    }
}

impl Vehicle for Car {
    fn have_ac(&self) -> bool {true}
    // fn is_electric(&self) -> bool {false};
}

trait Vehicle {
    fn have_ac(&self) -> bool;
    fn is_electric(&self) -> bool {false}
}