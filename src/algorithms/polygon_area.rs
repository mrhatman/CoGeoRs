//!Algorithms for calculating area of a set of points for a polygon.

use crate::primatives2d::{Point2D};
use num_traits::{Float,Zero};

///Preforms the Shoelace/Gauss Area Algorithm on a iterator of points to calculate area.
pub fn shoelace<T:Float+Zero,I>(mut points : I ) -> T  where I : Iterator<Item = Point2D<T>>{
	//https://en.wikipedia.org/wiki/Shoelace_formula
	
	let mut last_point = points.next().unwrap();
	let first_point = last_point;
	
	let mut area = T::zero();
	for point in points
	{
		area =area+ (point.x + last_point.x)*(point.y - last_point.y);
		last_point = point;
	}
	area =area+ (first_point.x + last_point.x)*(first_point.y - last_point.y);
	
	area.abs() / T::from(2).unwrap()

}





#[cfg(test)]
mod algorithms_test {
	use super::*;
	
    #[test]
    fn shoelace_test() {
		let polygon = vec!(
			Point2D::new(0.0,0.0),
			Point2D::new(0.0,1.0),
			Point2D::new(1.0,1.0),
			Point2D::new(1.0,0.0)
		); 
		assert_eq!(shoelace(polygon.into_iter()),1.0 );		
		
		let polygon = vec!(
			Point2D::new(-1.0,-1.0),
			Point2D::new(-1.0, 1.0),
			Point2D::new( 1.0, 1.0),
			Point2D::new( 1.0,-1.0)
		); 
		assert_eq!(shoelace(polygon.into_iter()),4.0 );		
		let polygon = vec!(
			Point2D::new(-1.0,-1.0),
			Point2D::new( 1.0,-1.0),
			Point2D::new( 1.0, 1.0),
			Point2D::new(-1.0, 1.0)
		); 
		assert_eq!(shoelace(polygon.into_iter()),4.0 );		
		let polygon = vec!(
			Point2D::new(-1.0,-1.0),
			Point2D::new(-1.0, 0.0),
			Point2D::new( 0.0, 0.0),
			Point2D::new( 0.0, 1.0),
			Point2D::new( 1.0, 1.0),
			Point2D::new( 1.0,-1.0)
		); 
		assert_eq!(shoelace(polygon.into_iter()),3.0 );			
		
		let polygon = vec!(
			Point2D::new(0.0,0.0),
			Point2D::new(10.0, 0.0),
			Point2D::new( -10.0,100.0),
		); 
		assert_eq!(shoelace(polygon.into_iter()),500.0 );
	}    

}
