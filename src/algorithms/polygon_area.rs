//!Algorithms for calculating area of a set of points for a polygon.

use crate::primatives2d::{Point2D};
use num_traits::{Float,Zero};

///Preforms the Shoelace/Gauss Area Algorithm on a set of points to calculate area.
pub fn shoelace<T:Float+Zero>(points : &[Point2D<T>]) -> T {
	//https://en.wikipedia.org/wiki/Shoelace_formula
	
	let mut area = T::zero();
	for q in 0..points.len()-1
	{
		area =area+ (points[q+1].x + points[q].x)*(points[q+1].y - points[q].y);
	}
	area =area+ (points[0].x + points[points.len()-1].x)*(points[0].y - points[points.len()-1].y);
	
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
		assert_eq!(shoelace(&polygon),1.0 );		
		
		let polygon = vec!(
			Point2D::new(-1.0,-1.0),
			Point2D::new(-1.0, 1.0),
			Point2D::new( 1.0, 1.0),
			Point2D::new( 1.0,-1.0)
		); 
		assert_eq!(shoelace(&polygon),4.0 );		
		let polygon = vec!(
			Point2D::new(-1.0,-1.0),
			Point2D::new( 1.0,-1.0),
			Point2D::new( 1.0, 1.0),
			Point2D::new(-1.0, 1.0)
		); 
		assert_eq!(shoelace(&polygon),4.0 );		
		let polygon = vec!(
			Point2D::new(-1.0,-1.0),
			Point2D::new(-1.0, 0.0),
			Point2D::new( 0.0, 0.0),
			Point2D::new( 0.0, 1.0),
			Point2D::new( 1.0, 1.0),
			Point2D::new( 1.0,-1.0)
		); 
		assert_eq!(shoelace(&polygon),3.0 );			
		
		let polygon = vec!(
			Point2D::new(0.0,0.0),
			Point2D::new(10.0, 0.0),
			Point2D::new( -10.0,100.0),
		); 
		assert_eq!(shoelace(&polygon),500.0 );
	}    

}
