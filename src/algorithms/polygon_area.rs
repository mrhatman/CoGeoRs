//!Algorithms for calculating area of a set of points for a polygon.

use crate::primatives2d::{Point2D};
use num_traits::{Float,Zero};
use std::ops::Deref;

///Preforms the Shoelace/Gauss Area Algorithm on a iterator of points to calculate area.
pub fn shoelace<'a,T:Float+Zero+'a,I,P>(mut points : I ) -> T where
	I : Iterator<Item = P>,
	P : Deref<Target = Point2D<T>> + 'a{
	//https://en.wikipedia.org/wiki/Shoelace_formula
	
	let mut last_point = *points.next().unwrap();
	let first_point = last_point;
	
	let mut area = T::zero();
	for point in points
	{
		area =area+ (point.x + last_point.x)*(point.y - last_point.y);
		last_point = point.clone();
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
		assert_eq!(shoelace(polygon.iter()),1.0 );		
		assert_eq!(shoelace(polygon.iter()),1.0 );		
		
		let polygon = vec!(
			Point2D::new(-1.0,-1.0),
			Point2D::new(-1.0, 1.0),
			Point2D::new( 1.0, 1.0),
			Point2D::new( 1.0,-1.0)
		); 
		assert_eq!(shoelace(polygon.iter()),4.0 );		
		let polygon = vec!(
			Point2D::new(-1.0,-1.0),
			Point2D::new( 1.0,-1.0),
			Point2D::new( 1.0, 1.0),
			Point2D::new(-1.0, 1.0)
		); 
		assert_eq!(shoelace(polygon.iter()),4.0 );		
		let polygon = vec!(
			Point2D::new(-1.0,-1.0),
			Point2D::new(-1.0, 0.0),
			Point2D::new( 0.0, 0.0),
			Point2D::new( 0.0, 1.0),
			Point2D::new( 1.0, 1.0),
			Point2D::new( 1.0,-1.0)
		); 
		assert_eq!(shoelace(polygon.iter()),3.0 );			
		
		let polygon = vec!(
			Point2D::new(0.0,0.0),
			Point2D::new(10.0, 0.0),
			Point2D::new( -10.0,100.0),
		); 
		assert_eq!(shoelace(polygon.iter()),500.0 );
	}    

}
