
use std::ops::Range;

use plotters::prelude::*;
use time::macros::datetime;
use time::{PrimitiveDateTime, Duration};
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;



fn main() -> ()  { //Result<(), Box<dyn std::error::Error>>

    
    let t_initial: PrimitiveDateTime = datetime!(2010-06-21 15:30:00); 
    let latitude: f64 = 40.0;
    let longitude:f64 = -86.5;
    let timezone:i8 = -7;

    let sp: SunPosition = sun_position(t_initial,latitude,longitude,timezone);
    
    
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
        azimuth_angle: {:?},
        sun up?: {:?}",
        sp.tot_julday,sp.jul_century,sp.geo_mean_long,sp.geo_mean_anom,sp.eccentricity,
        sp.sun_center,sp.sun_true_long, sp.mean_eclip_obliq,sp.obliq_corr, sp.sun_app_long,sp.sun_declin,sp.var_y,
        sp.eq_of_time_minutes, sp.minutes_past_midnight, sp.true_solar_time, sp.hour_angle,sp.solar_zenith_angle,sp.elev_angle,
        sp.azimuth_angle, sp.sunup);

    println!("refraction angle: {}, Reflectance: {}, solar radiation: {}",sp.refracted().to_degrees(), sp.reflection(), sp.solar_radiation());
        let dates = get_dt_range(datetime!(2010-01-01 00:00:00), datetime!(
        2010-01-02 00:00:00), 86400);
    

    let mut positions:Vec<SunPosition> = Vec::with_capacity(dates.len());
    

    for date in dates.iter(){
        positions.push(sun_position(*date, latitude, longitude, timezone))
    };

    let mut elev_angle:Vec<f64> = Vec::with_capacity(positions.capacity());
   for position in positions.iter(){
        elev_angle.push(position.elev_angle)
   }

   let mut indices: Vec<f64> = Vec::with_capacity(positions.capacity());
   for index in 0..positions.iter().len(){
    indices.push(index as f64);
   }

   let mut points: Vec<(f64,f64)> = Vec::new();
   for item in indices.iter(){
    points.push((*item, elev_angle[*item as usize]))
   }

   let mut radvec: Vec<(f64, f64)> = Vec::new();
   let mut rads: Vec<f64> = Vec::with_capacity(positions.capacity());
   for position in positions.iter(){
        rads.push(position.solar_radiation())
   }
   for item in indices.iter(){
    radvec.push((*item,rads[*item as usize]))
   }

   let _ = plot("./radiation.png", 2000,2000,radvec, 0.0..86400.0, 0.0..1360.0);

//     let root = BitMapBackend::new("./test.png", 
//     (10000, 2000)).into_drawing_area();
//     root.fill(&WHITE)?;
//     let mut chart = ChartBuilder::on(&root)
//         .x_label_area_size(900)
//         .y_label_area_size(900)
//         .build_cartesian_2d(0f64..1000000f64, -180f64..180f64)?;

//     chart
//         .configure_mesh()
//         .x_labels(30)
//         .y_labels(30)
//         .draw()?;

//     chart
//         .draw_series(LineSeries::new(points, &BLACK))?;

//     chart
//         .configure_series_labels()
//         .background_style(BLACK.mix(0.8))
//         .border_style(BLACK)
//         .draw()?;

//     root.present()?;

//     Ok(())
}
 
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
    azimuth_angle: f64, sunup:bool
}

impl SunPosition{
    fn solar_radiation(&self) -> f64{ //TODO: test where there are holes in the solar radiation function by making a plot for it.
        match self.sunup { true =>{
        let airmass:f64 = 1.0/self.solar_zenith_angle.to_radians().cos();
        let interior_exp:f64 = airmass.abs().powf(0.678);
        let radiation: f64 = 1353.0 *(0.7_f64.powf(interior_exp));
        radiation
    }
    ,false => {0.0}
    }}

    
    fn reflection(&self) -> f64{ //the model doesn't work if your ZA is greater than 90. Need some logic to handle. 
        
        match self.sunup{ 

            true =>
            {
            let angle:f64 = self.solar_zenith_angle.to_radians();
            let n_i = 1.000;
            let n_t = 1.333;
            let r_s: f64 = ((n_i * angle.cos()) - (n_t * self.refracted().cos())) / ((n_i*angle.cos()) + (n_t*self.refracted().cos()));
            let r_p: f64 = ((n_i*self.refracted().cos()) - (n_t*angle.cos())) / ((n_i*self.refracted().cos()) + (n_t * angle.cos()));
            let r: f64 = 0.5 * (r_s + r_p);
            let reflectance:f64 = r.powi(2);
            reflectance

        },

            false => {0.0}}
    }

    fn refracted(&self)->f64{
        match self.sunup {
            true => {//snell's law dictates that sin(a1)/n_21 = sin(a2)
            let n_i:f64 = 1.000;
            let n_t:f64 = 1.333;
            let ratio:f64 = n_i/n_t;
            let refracted = self.solar_zenith_angle.to_radians().sin()* ratio;
            let out = refracted.asin();
            out
        },
            false =>{0.0}
        }
        
    }
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

let azimuth_angle: f64 = //TODO: Correct this part

if hour_angle > 0.0 {
    (((latitude.to_radians().sin() * solar_zenith_angle.to_radians().sin() - sun_declin.to_radians().sin())
    /
    (latitude.to_radians().cos() * solar_zenith_angle.to_radians().sin()))
    .acos().to_degrees()
    + 180.0) % 360.0
} else {
    (540.0 - ((latitude.to_radians().sin() * solar_zenith_angle.to_radians().cos() - sun_declin.to_radians().sin())
    /
    (latitude.to_radians().cos() * solar_zenith_angle.to_radians().sin())).acos().to_degrees()) % 360.0


};

let sunup:bool = if solar_zenith_angle <= 90.0 {true} else {false};

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
    azimuth_angle:azimuth_angle, sunup:sunup
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

fn get_dt_range(t_initial:PrimitiveDateTime, t_final:PrimitiveDateTime, points: u32) -> 
Vec<PrimitiveDateTime> {

let timegap:Duration = (t_final - t_initial).abs();
let interval_res:Duration = timegap
    .checked_div(
        points.try_into().unwrap())
        .unwrap_or_else(||{Duration::new(0,0)});

let mut datevec: Vec<PrimitiveDateTime> = Vec::with_capacity(points.try_into().unwrap());

let mut counter:u32 = points;
datevec.push(t_initial);
counter -=1;

while counter > 0 {
    let time_point:PrimitiveDateTime = 
    t_initial.saturating_add((points-counter) * interval_res);
    datevec.push(time_point);
    counter -=1;
}
datevec.push(t_final);

datevec


}

fn plot(path:&str, dimx:u32, dimy:u32, datavec:Vec<(f64,f64)>, xcart:Range<f64>, ycart:Range<f64>) -> Result<(), Box<dyn std::error::Error>> { 

    let root = BitMapBackend::new(path, 
    (dimx, dimy)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(900)
        .y_label_area_size(900)
        .build_cartesian_2d(xcart, ycart)?;

    chart
        .configure_mesh()
        .x_labels(30)
        .y_labels(30)
        .draw()?;

    chart
        .draw_series(LineSeries::new(datavec, &BLACK))?;

    chart
        .configure_series_labels()
        .background_style(BLACK.mix(0.8))
        .border_style(BLACK)
        .draw()?;

    root.present()?;

    Ok(())

}

//Next item of business: calculate the heating which happens for the radiation - start in 1-d w/ a control surface
// also: setting up the box model for the lake
// also: refraction Done✅
// also: reflection from the surface Done ✅


//A lot of this stuff depends on the same fixed sun position parameters. It would be ideal to
//get all the sun params into a unified position in memory and then access all of it from each
//function as a thread. This would make it a lot faster to calculate the later steps. 

//TODO: Put the attributes you want in a giant polars table