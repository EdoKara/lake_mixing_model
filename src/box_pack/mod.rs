pub mod box_pack{
    use shapefile::record::polygon::GenericPolygon;
    use shapefile::Error;


    
    pub fn input_lake_shapes(filepath:&str) -> Result<Vec<GenericPolygon<shapefile::Point>>, Error> {

        /* This function takes an inpute filepath (stringslice) and uses the shapefile package to read
        the assumed shapefile. It either returns a vector of shapes or an error, depending on whether it
        was successful at actually reading in the lake shapes to pack. 
        
         */

        let reader = shapefile::ShapeReader::from_path(filepath)?;
        let polygons = reader.read_as::<shapefile::Polygon>()?;

        return Ok(polygons) 

        

    }


    pub fn select_lake_shape(index:usize, inputresult: Result<Vec<GenericPolygon<shapefile::Point>>, Error>) -> Option<GenericPolygon<shapefile::Point>>  {

        /* This function takes an index and an input set of lake shapes and outputs an Option with either the selected GenericPolygon or None. 
        If the input result from `input_lake_shapes()` is OK, it will unwrap the `Result<_,_>` to a `Vec<...>` and try to index it. 

        the `.get()` method returns an `Option<T>`. This `Option<T>` is then matched to return either the successful return polygon or None to the calling context. 

        If the input result is not ok, this function also returns `None`. 
        
         */

       if inputresult.is_ok(){
            let set_of_shapes = inputresult.unwrap().to_owned();
            let indexresult = set_of_shapes.get(index);
            match indexresult{
                Some(i) => {
                    let output = i;
                    Some(output.to_owned())
            },
                None => {
                    println!("Index not found in the vector! The vector's length is {}", set_of_shapes.len());
                    None}
            }
       } else {
            println!("Input read invalid!");
            None}
    
    }
}

