#[derive(Debug)] //allows to print stuff that hasn't been defined using {:?}
struct Appliance {
    name: String,
    power_in_watts: f32,
}

impl Appliance {
    fn calculate_watt_hours(&self, time: f32) -> f32 {
        let watt_hours: f32 = self.power_in_watts * time;
        return watt_hours;
    }
}

#[derive(Debug)] //allows to print stuff that hasn't been defined using {:?}
struct SolarArray {
    power_in_watts: f32,
    count: i32,
}

impl SolarArray {
    fn calculate_watt_hours(&self, time: f32) -> f32 {
        let watt_hours: f32 = self.power_in_watts * self.count as f32 * time;
        return watt_hours;
    }

   fn can_support_appliance_needs(&self, appliances: Vec<Appliance>, sun_time: f32, appliance_running_time: f32) -> bool {
        let mut total_appliance_wh : f32 = 0.0;
        for appliance in appliances.iter() {
            total_appliance_wh += appliance.calculate_watt_hours(appliance_running_time);
        }
        let total_solar_wh : f32 = self.calculate_watt_hours(sun_time);
        println!("Solar kwh output is {0}wh, Appliance needs are {1} wh",total_solar_wh, total_appliance_wh);
        return total_solar_wh >= total_appliance_wh;
   }
}

fn calculate_watt_hours(power: f32, time: f32) -> f32 {
    let watt_hours: f32 = power * time;
    return watt_hours;
}

fn main() {
    let test = calculate_watt_hours(10.0, 5.0);
    println!("{0}", test);

    let mut appliance_list = vec![];
    
    // Different ways to create structs
    let n = String::from("Toaster");
    let p = 900.0;
    let toaster = Appliance {
        name: n,
        power_in_watts: p,
    };
    let kettle = Appliance {
        name: String::from("Kettle"),
        power_in_watts: 2000.0,
    };
    let lightbulbs = Appliance{ name: String::from("Lightbulbs"), power_in_watts: 400.0};

    appliance_list.push(toaster);
    appliance_list.push(kettle);
    appliance_list.push(lightbulbs);
    println!("{:?}", appliance_list);

    let solar_array_1 = SolarArray {
        power_in_watts: 300.0,
        count: 10,
    };
    let watt_hours = solar_array_1.calculate_watt_hours(5.0);
    println!("{0}", watt_hours);
    println!("{:?}", solar_array_1);

    let sun_time : f32 = 5.0;
    let appliance_running_time : f32 = 6.0;
    let can_it_support : bool = solar_array_1.can_support_appliance_needs(appliance_list,sun_time,appliance_running_time);
    println!("{0}", can_it_support);
}
