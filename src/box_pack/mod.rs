pub mod box_pack{
    use shapefile::record::polygon::GenericPolygon;
    use shapefile::Error;

    
    pub fn input_lake_shapes(filepath:&str) -> Result<Vec<GenericPolygon<shapefile::Point>>,Error> {

        /* This function takes an inpute filepath (stringslice) and uses the shapefile package to read
        the assumed shapefile. It either returns a vector of shapes or an error, depending on whether it
        was successful at actually reading in the lake shapes to pack. 
        
         */

        let reader = shapefile::ShapeReader::from_path(filepath)?;
        let polygons = reader.read_as::<shapefile::Polygon>()?;
        return Ok(polygons)
    }


}