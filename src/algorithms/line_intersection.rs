//!Algorithms for calculating intersection points of line sets.
use crate::primatives2d::{Point2D,Line2D};
use num_traits::Float;

///Preforms the Bentley Ottmann Line Intersection Algorithm on a set of lines.
pub fn bentley_ottmann<T>(lines : &[Line2D<T>]) -> Vec<Point2D<T>> where T: Float{
	//https://en.wikipedia.org/wiki/Bentley%E2%80%93Ottmann_algorithm
	
	Vec::new()
}


#[cfg(test)]
mod algorithms_test {
	use super::*;
	
    #[test]
    fn bentley_ottmann_test() {
        let lines = vec!(
			Line2D::new(Point2D::new(0.0,0.0),Point2D::new(2.0,2.0)),
			Line2D::new(Point2D::new(1.0,0.0),Point2D::new(0.0,1.0)),
		);
		let expected_results = vec!(
			Point2D::new(1.0,1.0),
		);    
		
		assert_eq!(bentley_ottmann(&lines),expected_results);
	}    

}
