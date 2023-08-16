
use plotters::prelude::*;
use iter_num_tools::lin_space;
use time::macros::datetime;
use time::{PrimitiveDateTime, Duration};
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;



fn main() -> ()  { //Result<(), Box<dyn std::error::Error>>

    
    let t_initial = datetime!(2010-01-01 12:00:00); //lin_space(dec!(0.0)..=dec!(2.5), 500);
    let latitude: f64 = 40.0;
    let longitude:f64 = -86.5;
    let timezone:i8 = -7;

    let sp = sun_position(t_initial,latitude,longitude,timezone);

    
    println!("julian days: {:?}, 
    Julian Century: {:?},
    geo_mean_long: {:?},
    geo_mean_anom: {:?},
    eccentricity: {:?},
    sun_center: {:?},
    sun_true_long: {:?},
    mean_eclip_obliq: {:?},
    obliq_corr: {:?},
    sun_app_long: {:?},
    sun_declin: {:?},
    var_y: {:?},
    eq_of_time_minutes: {:?},
    minutes_past_midnight: {:?},
    true_solar_time: {:?},
    hour_angle: {:?},
    solar_zenith_angle: {:?},
    elev_angle: {:?},
    azimuth_angle: {:?}",
    sp.tot_julday,sp.jul_century,sp.geo_mean_long,sp.geo_mean_anom,sp.eccentricity,
    sp.sun_center,sp.sun_true_long, sp.mean_eclip_obliq,sp.obliq_corr, sp.sun_app_long,sp.sun_declin,sp.var_y,
    sp.eq_of_time_minutes, sp.minutes_past_midnight, sp.true_solar_time, sp.hour_angle,sp.solar_zenith_angle,sp.elev_angle,
    sp.azimuth_angle)

    // let mut angle: Vec<(f64,f64)> = Vec::new();
    // let mut rad: Vec<(f64,f64)> = Vec::new();
    // //let mut ar: Vec<(f64,f64)> = Vec::new();

    // for val in t_initial { 
        
    //     angle.push(
    //         ( //opens tuple
    //             val
    //             .to_f64()
    //             .unwrap(),
    //              sun_position(val, latitude, longitude, timezone).eq_of_time_minutes) //closes tuple
    //         ); //remember to make a tuple of X,Y to plot with instead of just 

    //     rad.push(
    //         (
    //             val
    //             .to_f64()
    //             .unwrap(),
    //             solar_radiation(val, latitude, longitude, timezone)
    //         )
    //     )
    // };

    // let root = BitMapBackend::new("./test.png", 
    // (2000, 2000)).into_drawing_area();
    // root.fill(&WHITE)?;
    // let mut chart = ChartBuilder::on(&root)
    //     .x_label_area_size(90)
    //     .y_label_area_size(90)
    //     .build_cartesian_2d(0f64..2.5f64, -3f64..-4f64)?;

    // chart
    //     .configure_mesh()
    //     .x_labels(30)
    //     .y_labels(30)
    //     .draw()?;

    // chart
    //     .draw_series(LineSeries::new(angle, &BLACK))?;

    // chart
    //     .configure_series_labels()
    //     .background_style(BLACK.mix(0.8))
    //     .border_style(BLACK)
    //     .draw()?;

    // root.present()?;

    // Ok(())
}
 


// fn solar_radiation(t_initial: Decimal, latitude:f64, longitude:f64, timezone:i8) -> f64 {
// //calculate the incident solar radiation for a lake based upon the time of the year. 

// //Assuming no cloud cover, N hemisphere, and no other influencing factors (i.e. the raw solar radiation coming in)

// //this boils down mostly to angle.


// //const CF:f64 = 180.0/PI; //conversion factor between radians and degrees.


// let eangle:f64 = sun_position(t_initial, latitude, longitude, timezone).elev_angle;

// let airmass:f64 = 1.0/(eangle.cos());

// let radiation:f64 = 1353.0 * (0.7_f64).powf((airmass).powf(0.678_f64));

// radiation
// }

struct SunPosition{
    tot_julday:Decimal, jul_century:f64,
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





fn sun_position(t_initial:PrimitiveDateTime, latitude:f64, longitude:f64, timezone:i8) -> SunPosition{

//we're going to do some more sophisticated stuff here; I want to find the azimuth and 
//elevation angles and use that as inputs to the function. 

let tot_julday:Decimal = get_julian_day(t_initial, timezone); //works as intended
let jul_century:f64 = ((tot_julday- dec!(2451545)) / dec!(36525)).to_f64().unwrap(); //conversion to julian century. works as intended

//=MOD(280.46646+G2*(36000.76983+G2*0.0003032),360)
let geo_mean_long:f64 = 
(280.46646 + jul_century * (36000.76983 + jul_century * 0.0003032)) % 360.0;

//=357.52911+G2*(35999.05029-0.0001537*G2)

let geo_mean_anom: f64  = 357.52911+(jul_century.to_f64().unwrap()) //works right
    *(35999.05029-0.0001537*jul_century.to_f64().unwrap());


// =0.016708634-G2*(0.000042037+0.0000001267*G2)
    
let eccentricity:f64 = 0.016708634 - //works
    jul_century.to_f64().unwrap() *
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


let sun_declin:f64 = ((obliq_corr.to_radians().sin()) * //works
    (sun_app_long
        .to_radians()
        .sin())
        ).asin()
        .to_degrees();

let var_y:f64 = ((obliq_corr/2.0).to_radians()).tan()
                *((obliq_corr/2.0).to_radians()).tan(); //works



let eq_of_time_minutes: f64 = 4.0 * ( // the parentheses order is very particular here; had to check
(var_y * (geo_mean_long.to_radians()*2.0).sin()) - //equation builder in libreoffice to make sure I got it right.
(2.0*eccentricity*geo_mean_anom.to_radians().sin()) +
(4.0*eccentricity*var_y*geo_mean_anom.to_radians().sin() * (2.0*geo_mean_long.to_radians()).cos()) -
(0.5*var_y*var_y*(4.0*geo_mean_long.to_radians()).sin()) - 
(1.25*eccentricity*eccentricity* (2.0*geo_mean_anom.to_radians()).sin())
).to_degrees();

let hms = t_initial.as_hms();
let hours:Decimal = <u8 as Into<Decimal>>::into(hms.0) / dec!(24);
let minutes:Decimal = <u8 as Into<Decimal>>::into(hms.1)/dec!(1440);
let seconds:Decimal = <u8 as Into<Decimal>>::into(hms.2)/dec!(86400);

let minutes_past_midnight:Decimal = (hours + minutes + seconds)*dec!(1440);

let true_solar_time: f64 = (
    ((minutes_past_midnight.to_f64().unwrap() + eq_of_time_minutes) + 4.0 * longitude) -
    (60.0 * timezone.to_f64().unwrap()))
    % 1400.0;

let hour_angle: f64 = (true_solar_time/4.0)-180.0;

let solar_zenith_angle: f64 = (((latitude.to_radians().sin()
    *(sun_declin.to_radians().sin())) + 
    (latitude.to_radians().cos())*sun_declin.to_radians().cos()*
    hour_angle.to_radians().cos())   
    ).acos().to_degrees();

let elev_angle: f64 = 90.0 - solar_zenith_angle;

let azimuth_angle: f64 = 

if hour_angle > 0.0 {
    ((((latitude.to_radians().sin() * solar_zenith_angle.to_radians().cos()) - 
    sun_declin.to_radians().sin()) /
    latitude.to_radians().cos() * solar_zenith_angle.to_radians().sin()).acos().to_degrees() + 180.0) % 360.0
} else {
(540.0 -
((((latitude.to_radians().sin() * solar_zenith_angle.to_radians().cos()) -
sun_declin.to_radians().sin()) / (latitude.to_radians().cos()* solar_zenith_angle.to_radians().sin()))
.acos().to_degrees())) % 360.0};

let out = SunPosition{
    tot_julday:tot_julday, jul_century:jul_century,
    geo_mean_long:geo_mean_long, geo_mean_anom:geo_mean_anom,
    eccentricity:eccentricity, sun_center:sun_center,
    sun_true_long:sun_true_long, mean_eclip_obliq:mean_eclip_obliq,
    obliq_corr:obliq_corr, sun_app_long:sun_app_long,
    sun_declin:sun_declin, var_y:var_y,
    eq_of_time_minutes:eq_of_time_minutes, minutes_past_midnight: minutes_past_midnight.to_f64().unwrap(),
    true_solar_time:true_solar_time, hour_angle:hour_angle,
    solar_zenith_angle: solar_zenith_angle, elev_angle:elev_angle,
    azimuth_angle:azimuth_angle
};

out
}

fn get_julian_day(datetime:PrimitiveDateTime, timezone:i8) -> Decimal {

    // get julian day with the same methodology as the 
    //NOAA paper; implements fractional julian days. 

    let baseday: i32 = datetime.to_julian_day();
    let hms = datetime.as_hms();
    let hours:Decimal = <u8 as Into<Decimal>>::into(hms.0) / dec!(24);
    let minutes:Decimal = <u8 as Into<Decimal>>::into(hms.1)/dec!(1440);
    let seconds:Decimal = <u8 as Into<Decimal>>::into(hms.2)/dec!(86400);
    let frac_julianday: Decimal = hours + minutes + seconds;

    let tz:Decimal = <i8 as Into<Decimal>>::into(timezone);
    let cf:Decimal = tz/dec!(24);

    let julday = <i32 as Into<Decimal>>::into(baseday) + frac_julianday;
    julday
    
}

