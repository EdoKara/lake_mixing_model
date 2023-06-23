
fn main() {

let t_initial:f64 = 150.10248;
let latitude:f64 = 39.5;

let angle = sun_angle(&t_initial, &latitude);
let rad = solar_radiation(&t_initial, &latitude);

println!("Solar radiation is {rad} watts per meter squared. sun angle at this time is {angle} degrees.")    
}

fn solar_radiation(t_initial: &f64, latitude:&f64) -> f64 {
//calculate the incident solar radiation for a lake based upon the time of the year. 

//Assuming no cloud cover, N hemisphere, and no other influencing factors (i.e. the raw solar radiation coming in)

//this boils down mostly to angle.

let θ:f64 = sun_angle(&t_initial, &latitude);

let airmass:f64 = 1_f64/(θ.cos());

let radiation:f64 = 1353.0 * (0.7_f64).powf((airmass).powf(0.678_f64));

return radiation
}

fn sun_angle(t_initial:&f64, latitude:&f64) -> f64{

//the definitions I've seen for elevation angle of the sun for time are based on time past the spring equinox. 
//I'm going to define it in hours for some simplicity.

let sun_angle:f64 = 360.0*((t_initial*24.0) % 24.0);

let horiz_angle:f64 = 23.5*(360_f64*(t_initial/365.25)).sin();

let η:f64 = 90.0 - (
    (&latitude.cos()*horiz_angle.cos()*sun_angle.cos()) + 
    (horiz_angle.sin()*&latitude.sin())
).acos();

η
}