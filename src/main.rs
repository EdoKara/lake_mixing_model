
use plotters::prelude::*;
use iter_num_tools::lin_space;
use time::macros::datetime;
use time::{PrimitiveDateTime};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let t_initial = lin_space(0.0..=1.0, 24);
    let latitude = 39.2;
    
    let mut angle: Vec<(f64,f64)> = Vec::new();
    let mut rad: Vec<(f64,f64)> = Vec::new();
    let mut ar: Vec<(f64,f64)> = Vec::new();

    for val in t_initial { 
        angle.push((val, sun_angle(val, latitude))); //remember to make a tuple of X,Y to plot with instead of just 
        rad.push((val, solar_radiation(val, latitude)));// a series. 
        ar.push((val, solar_radiation(val, latitude)));
    };

    let root = BitMapBackend::new("./test.png", 
    (1920, 1080)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .build_cartesian_2d(0f64..1f64, -90f64..90f64)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(angle, &BLACK))?;

    chart
        .configure_series_labels()
        .background_style(WHITE.mix(0.8))
        .border_style(BLACK)
        .draw()?;

    root.present()?;

    Ok(())
}

    


fn solar_radiation(t_initial: f64, latitude:f64) -> f64 {
//calculate the incident solar radiation for a lake based upon the time of the year. 

//Assuming no cloud cover, N hemisphere, and no other influencing factors (i.e. the raw solar radiation coming in)

//this boils down mostly to angle.


//const CF:f64 = 180.0/PI; //conversion factor between radians and degrees.


let θ:f64 = sun_angle(t_initial, latitude);

let airmass:f64 = 1_f64/(θ.cos());

let radiation:f64 = 1353.0 * (0.7_f64).powf((airmass).powf(0.678_f64));

radiation
}

fn sun_angle(t_initial:f64, latitude:f64, longitude:f64) -> f64{

//we're going to do some more sophisticated stuff here; I want to find the azimuth and 
//elevation angles and use that as inputs to the function. 

//initial time is given in days since Jan 1

//be procedural and start with what day it is; this informs a lot of the position calculations
//that NOAA does.

const start_time: PrimitiveDateTime = datetime!(2023-01-01 00:00:00.00);
const hours_to_seconds:f64 = 24.0*60.0*60.0;
let seconds_initial = t_initial*hours_to_seconds;
let time_since_start: PrimitiveDateTime = start_time + seconds_initial.seconds(); 




}
