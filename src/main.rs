
use plotters::prelude::*;
use iter_num_tools::lin_space;
use time::macros::datetime;
use time::{PrimitiveDateTime, Duration};
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;



fn main() -> Result<(), Box<dyn std::error::Error>> {

    let t_initial = lin_space(dec!(0.0)..=dec!(2.5), 500);
    let latitude: f64 = 39.2;
    let longitude:f64 = -86.5;
    let timezone:i8 = -5;
    
    let mut angle: Vec<(f64,f64)> = Vec::new();
    let mut rad: Vec<(f64,f64)> = Vec::new();
    //let mut ar: Vec<(f64,f64)> = Vec::new();

    for val in t_initial { 
        
        angle.push(
            ( //opens tuple
                val
                .to_f64()
                .unwrap(),
                 sun_position(val, latitude, longitude, timezone).sun_app_long) //closes tuple
            ); //remember to make a tuple of X,Y to plot with instead of just 

        rad.push(
            (
                val
                .to_f64()
                .unwrap(),
                solar_radiation(val, latitude, longitude, timezone)
            )
        )
    };

    let root = BitMapBackend::new("./test.png", 
    (2000, 2000)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(90)
        .y_label_area_size(90)
        .build_cartesian_2d(0f64..2.5f64, 270f64..290f64)?;

    chart
        .configure_mesh()
        .x_labels(30)
        .y_labels(30)
        .draw()?;

    chart
        .draw_series(LineSeries::new(angle, &BLACK))?;

    chart
        .configure_series_labels()
        .background_style(BLACK.mix(0.8))
        .border_style(BLACK)
        .draw()?;

    root.present()?;

    Ok(())
}
 


fn solar_radiation(t_initial: Decimal, latitude:f64, longitude:f64, timezone:i8) -> f64 {
//calculate the incident solar radiation for a lake based upon the time of the year. 

//Assuming no cloud cover, N hemisphere, and no other influencing factors (i.e. the raw solar radiation coming in)

//this boils down mostly to angle.


//const CF:f64 = 180.0/PI; //conversion factor between radians and degrees.


let eangle:f64 = sun_position(t_initial, latitude, longitude, timezone).elev_angle;

let airmass:f64 = 1.0/(eangle.cos());

let radiation:f64 = 1353.0 * (0.7_f64).powf((airmass).powf(0.678_f64));

radiation
}

struct SunPosition{
    tot_julday:Decimal, jul_century:Decimal,
    geo_mean_long:f64, geo_mean_anom:f64,
    eccentricity:f64, sun_center:f64,
    sun_true_long:f64, mean_eclip_obliq:f64,
    obliq_corr:f64, sun_app_long:f64,
    sun_declin:f64, var_y:f64,
    eq_of_time_minutes: f64, minutes_past_midnight: f64,
    true_solar_time: f64, hour_angle: f64,
    solar_zenith_angle: f64, elev_angle: f64,
    azimuth_angle: f64
}

fn sun_position(t_initial:Decimal, latitude:f64, longitude:f64, timezone:i8) -> SunPosition{

//we're going to do some more sophisticated stuff here; I want to find the azimuth and 
//elevation angles and use that as inputs to the function. 

let tot_julday:Decimal = get_julian_day(t_initial); //works as intended
let jul_century:Decimal = (tot_julday
    - dec!(2451545)) / dec!(36525); //conversion to julian century. works as intended



let geo_mean_long:f64 = (280.46646 + (
    (jul_century.to_f64().unwrap())*((36000.76893+(jul_century.to_f64().unwrap()))*0.0003032)
))%360.0;

let geo_mean_anom: f64  = 357.52911+jul_century.to_f64().unwrap() //works right
    *(35999.05029-0.0001537*jul_century.to_f64().unwrap());

let eccentricity:f64 = 0.016708634- //works
    jul_century.to_f64().unwrap()*
    (0.000042037+0.0000001267*jul_century.to_f64().unwrap());

let sun_center:f64 = (geo_mean_anom.to_radians().sin() //works
    *(1.914602-jul_century.to_f64().unwrap()*
    (0.004817+0.000014*jul_century.to_f64().unwrap())))
    + (2_f64*geo_mean_anom.to_radians().sin() *
    (0.019993-0.000101*jul_century.to_f64().unwrap())) + 
    (3_f64*geo_mean_anom.to_radians().sin()*0.000289);

let sun_true_long:f64 = &geo_mean_long + &sun_center; //works


let sun_app_long:f64 = sun_true_long - 0.00569 - 0.00478 * 
125.04 - (1934.136 * (jul_century.to_f64().unwrap())).to_radians().sin(); //works


let mean_eclip_obliq:f64 = 23.0 + //works
    (26.0 +
        (
            (21.448-jul_century.to_f64().unwrap()*(
                46.815+jul_century.to_f64().unwrap()*(
                    0.00059-jul_century.to_f64().unwrap()*0.001813
                )
            )
        )
    )
    / 60.0
)
    /60.0;

let obliq_corr:f64 = mean_eclip_obliq + //works
    0.00256*(
        125.04-1934.136*jul_century
        .to_f64()
        .unwrap()
    )
    .to_radians()
    .cos();


let sun_declin:f64 = ((obliq_corr.to_radians().sin()) * 
    (sun_app_long
        .to_radians()
        .sin())
        ).asin()
        .to_degrees();

let var_y:f64 = obliq_corr.powf(2.0);

let eq_of_time_minutes: f64 =  (4.0*(var_y*2.0*geo_mean_long.to_radians()).to_degrees()-
    2.0*eccentricity*(geo_mean_anom.to_radians()).sin()+4.0*eccentricity*var_y*
    (geo_mean_anom.to_radians()).sin()*(geo_mean_anom.to_radians()).cos()
    -0.5*var_y*var_y*(geo_mean_anom.to_radians()).cos()-1.25*eccentricity*eccentricity*
    (2.0*(geo_mean_anom.to_radians()).sin())).to_degrees();

let minutes_past_midnight: f64 = tot_julday.trunc().to_f64().unwrap()*24.0*60.0; //converting back to minutes 

let true_solar_time: f64 = (
    (
        (
            minutes_past_midnight*1440.0
        )+eq_of_time_minutes+4.0
    )*(longitude-60.0
    )*timezone.to_f64()
    .unwrap()
) % 1400.0;

let hour_angle: f64 = if true_solar_time/4.0 < 0.0{
    (true_solar_time/4.0) - 180.0
} else {
    (true_solar_time/4.0) + 180.0
};

let solar_zenith_angle: f64 = (latitude.to_radians().sin().acos()
    *(sun_declin.to_radians().sin() + 
    latitude.to_radians().cos())*
    sun_declin.to_radians().cos()*
    hour_angle.to_radians().cos()
    ).to_degrees();

let elev_angle: f64 = 90.0 - solar_zenith_angle;

let azimuth_angle: f64 = if hour_angle < 0.0 {
    ((latitude.to_radians().sin() * solar_zenith_angle.to_radians().cos() - 
sun_declin.to_radians().cos()) / (latitude.to_radians().cos() * solar_zenith_angle.to_radians().sin())
.acos() + 180.0
    )
    .to_degrees()
    % 360.0
} else {
    (
        ((latitude.to_radians().sin() * solar_zenith_angle.to_radians().cos()) - sun_declin.to_radians().sin()).acos()
    / (latitude.to_radians().cos()*solar_zenith_angle.to_radians().sin())
    ).to_degrees() % 360.0
};

let out = SunPosition{
    tot_julday:tot_julday, jul_century:jul_century,
    geo_mean_long:geo_mean_long, geo_mean_anom:geo_mean_anom,
    eccentricity:eccentricity, sun_center:sun_center,
    sun_true_long:sun_true_long, mean_eclip_obliq:mean_eclip_obliq,
    obliq_corr:obliq_corr, sun_app_long:sun_app_long,
    sun_declin:sun_declin, var_y:var_y,
    eq_of_time_minutes:eq_of_time_minutes, minutes_past_midnight: minutes_past_midnight,
    true_solar_time:true_solar_time, hour_angle:hour_angle,
    solar_zenith_angle: solar_zenith_angle, elev_angle:elev_angle,
    azimuth_angle:azimuth_angle
};

out
}

fn get_julian_day(t_initial:Decimal) -> Decimal {

//initial time is given in days since Jan 1 2023

//be procedural and start with what day it is; this informs a lot of the position calculations
//that NOAA does.

    const START_TIME: PrimitiveDateTime = datetime!(2023-01-01 00:00:00.00);
    let days_to_seconds:Decimal = dec!(86400);
    
    let days_t_initial: i64 = t_initial.trunc().to_i64().unwrap();
    
    let days_t_initial_dur: Duration = Duration::days(days_t_initial);
    
    let seconds_t_initial: Duration = Duration::seconds_f64(
        ((t_initial - t_initial.trunc()) * days_to_seconds)
        .to_f64()
        .unwrap()
    );
    
    let julday: Decimal = Decimal::from_f64(
        (START_TIME + (days_t_initial_dur + seconds_t_initial))
        .to_julian_day()
        .to_f64()
        .unwrap())
        .unwrap();
       
    
    let tot_julday: Decimal = julday + t_initial.fract();

    return tot_julday
}

