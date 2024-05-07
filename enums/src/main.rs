enum CarType {
    Hatchback,
    Sedan,
    SUV
}

fn size_of_car(car:CarType) {
    match car {
        CarType::Hatchback => {
            println!("Small sized car");
        },
        CarType::Sedan => {
            println!("Medium sized car");
        },
        CarType::SUV => {
            println!("Larged sized car");
        },

    }
}

fn main() {
    size_of_car(CarType::SUV);
    size_of_car(CarType::Hatchback);
    size_of_car(CarType::Sedan);

    person_data();

}

// Match $ enum with data types

enum GenderCato {
    Name(String),
    _UserId(i32)
}

fn person_data() {
    let p1 = GenderCato::Name(String::from("Freaky coder"));

    match p1 {
        GenderCato::Name(val) => {
            println!("{}",val);
        }

        GenderCato::_UserId(val) => {
            println!("{}",val);
        }


    }
}