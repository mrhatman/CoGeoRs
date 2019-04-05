use crate::primatives2d::{Point2D,TurnDirection};
use num_traits::Float;


fn jarvis_march<T>(points : &Vec<Point2D<T>>) -> Vec<Point2D<T>> where T: Float{
	//https://en.wikipedia.org/wiki/Gift_wrapping_algorithm
	
	//find left most point
	let mut left_most_point = points[0];
	
	for &p in points.iter(){
		if p.x < left_most_point.x {
			left_most_point = p;
		}
		else if (p.x == left_most_point.x) && (p.y < left_most_point.y) {
			left_most_point = p;
		}
	}
	
	let mut hull = Vec::new();
	hull.push(left_most_point);
	
	let mut current_point = left_most_point;
	
	loop{
		let mut next_point = points[0];
		for &p in points.iter(){
			if ( next_point == current_point) || ( current_point.turn_direction(&next_point,&p) == TurnDirection::LeftTurn)
			{
				next_point = p;
			}
		}
		if next_point == left_most_point{
			break;
		}
		else{
			hull.push(next_point);
			current_point = next_point;
		}
		
		
	}
	
	hull
}


#[cfg(test)]
mod algorithms_test {
	use super::*;
	
    #[test]
    fn jarvis_march_test() {
        let points = vec!(
			Point2D::new(0.0,0.0),
			Point2D::new(0.0,1.0),
			Point2D::new(1.0,0.0),
			Point2D::new(1.0,1.0),
			Point2D::new(0.5,0.5)
		);        
		let expected_results = vec!(
			Point2D::new(0.0,0.0),
			Point2D::new(0.0,1.0),
			Point2D::new(1.0,1.0),
			Point2D::new(1.0,0.0)
		);    
		
		assert_eq!(jarvis_march(&points),expected_results);
	}    

}
