
use plotters::prelude::*;
use iter_num_tools::lin_space;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let t_initial = lin_space(0.0..=1.0, 1000);
    let latitude = 39.2;
    
    let mut angle: Vec<f64> = Vec::new();
    let mut rad: Vec<f64> = Vec::new();
    let mut ar: Vec<(f64,f64)> = Vec::new();

    for val in t_initial { 
        angle.push(sun_angle(&val, &latitude));
        rad.push(solar_radiation(&val, &latitude));
        ar.push((val, solar_radiation(&val, &latitude)));
    };

    let root = BitMapBackend::new("./test.png", 
    (1920, 1080)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .build_cartesian_2d(0f64..1f64, -100f64..1400f64)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(ar.iter().cloned(), &BLACK))?;

    chart
        .configure_series_labels()
        .background_style(WHITE.mix(0.8))
        .border_style(BLACK)
        .draw()?;

    root.present()?;

    Ok(())
}

    


fn solar_radiation(t_initial: &f64, latitude:&f64) -> f64 {
//calculate the incident solar radiation for a lake based upon the time of the year. 

//Assuming no cloud cover, N hemisphere, and no other influencing factors (i.e. the raw solar radiation coming in)

//this boils down mostly to angle.

const PI:f64 = std::f64::consts::PI;
const CF:f64 = 180.0/PI; //conversion factor between radians and degrees.


let θ:f64 = sun_angle(t_initial, latitude);

let airmass:f64 = 1_f64/(θ.cos()*CF);

let radiation:f64 = 1353.0 * (0.7_f64).powf((airmass).powf(0.678_f64));

radiation
}

fn sun_angle(t_initial:&f64, latitude:&f64) -> f64{

//the definitions I've seen for elevation angle of the sun for time are based on time past the spring equinox. 
//I'm going to define it in hours for some simplicity.

const PI:f64 = std::f64::consts::PI;
const CF:f64 = 180.0/PI; //conversion factor between radians and degrees.

let sun_angle:f64 = 360.0*(t_initial % 24.0);

let horiz_angle:f64 = 23.5*(360_f64*((t_initial/365.25).sin()*CF));

let η:f64 = 90.0 - ((
    (latitude.cos()*horiz_angle.cos()*sun_angle.cos()) + 
    (horiz_angle.sin()*latitude.sin())
).acos())*CF;

η
}
