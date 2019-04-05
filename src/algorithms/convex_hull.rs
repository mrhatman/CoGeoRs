//!Algorithms for creating convex hulls.
// from a set of Point2Ds.
//Results starts from the left most point and goes clockwise.

use crate::primatives2d::{Point2D,TurnDirection};
use num_traits::Float;


///Preforms the Jarvis March/Gift Wrapping Algorithm on a set of points.
pub fn jarvis_march<T>(points : &[Point2D<T>]) -> Vec<Point2D<T>> where T: Float{
	//https://en.wikipedia.org/wiki/Gift_wrapping_algorithm
	
	//find left most point
	let mut left_most_point = points[0];
	
	for &p in points.iter(){
		if p.x < left_most_point.x || ((p.x == left_most_point.x) && (p.y < left_most_point.y)) {
			left_most_point = p;
		}
	}
	
	let mut hull = Vec::new();
	hull.push(left_most_point);
	
	let mut current_point = left_most_point;
	
	loop{
		let mut next_point =points[0];
		for &p in points.iter(){
			if (next_point == current_point) || ( current_point.turn_direction(&next_point,&p) == TurnDirection::LeftTurn)
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

///Preforms the Monotone Chain Algorithm on a set of points.
pub fn monotone_chain<T>(points :&mut  Vec<Point2D<T>>) -> Vec<Point2D<T>> where T: Float{
	//https://en.wikibooks.org/wiki/Algorithm_Implementation/Geometry/Convex_hull/Monotone_chain
	
	points.sort_by(|a,b| a.x_then_y_partial_cmp(b).unwrap());
	
	let mut stack : Vec<Point2D<T>>= Vec::new();
	
	for p in points.iter(){

		while 
			stack.len() >= 2 &&
			(stack[stack.len()-1].turn_direction(&stack[stack.len()-2],p) == TurnDirection::RightTurn)
		{ 
			stack.pop();
		}
		
		stack.push(*p);
		
	}
	stack.pop();
	
	let last_len = stack.len();
	
	for &p in points.iter().rev(){
		while 
		
			stack.len() >= last_len+2 &&
			(stack[stack.len()-1].turn_direction(&stack[stack.len()-2],&p) == TurnDirection::RightTurn)
		{ 
			stack.pop();
		}
		
		stack.push(p);
		
	}
	
	stack.pop();
	
	stack
}

///Preforms the Graham Scan Algorithm on a set of points.
pub fn graham_scan<T>(points :&mut  Vec<Point2D<T>>) -> Vec<Point2D<T>> where T: Float{
	//https://en.wikibooks.org/wiki/Algorithm_Implementation/Geometry/Convex_hull/Monotone_chain
	
	//find left lowest most point
	let mut left_lowest_most_point = points[0];
	
	for &p in points.iter(){
		if p.x < left_lowest_most_point.x || ((p.x == left_lowest_most_point.x) && (p.y < left_lowest_most_point.y)) {
			left_lowest_most_point = p;
		}
	}
	
	points.sort_by(|a,b| a.rotation_point_cmp(b,&left_lowest_most_point).unwrap());
	
	let mut stack : Vec<Point2D<T>>= Vec::new();
	
	for &p in points.iter(){

		while 
			stack.len() >= 2 &&
			(stack[stack.len()-1].turn_direction(&stack[stack.len()-2],&p) == TurnDirection::RightTurn)
		{ 
			stack.pop();
		}
		
		stack.push(p);
		
	}

	
	stack
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
	
	#[test]
    fn monotone_chain_test() {
        let mut points = vec!(
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
		
		assert_eq!(monotone_chain(&mut points),expected_results);
	}	
	#[test]
    fn graham_scan_test() {
        let mut points = vec!(
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
		
		assert_eq!(graham_scan(&mut points),expected_results);
	}
	
	#[test]
    fn random_stress_test() {
		use rand::Rng;
	
        let mut points = vec!(
			Point2D::new(0.0,0.0),
			Point2D::new(0.0,1.0),
			Point2D::new(1.0,0.0),
			Point2D::new(1.0,1.0),
			Point2D::new(0.5,0.5)
		);    

		for _ in 0..100000{
			points.push( Point2D::new(rand::thread_rng().gen(),rand::thread_rng().gen()));
		}
		
		let expected_results = vec!(
			Point2D::new(0.0,0.0),
			Point2D::new(0.0,1.0),
			Point2D::new(1.0,1.0),
			Point2D::new(1.0,0.0)
		);    
		
		assert_eq!(graham_scan(&mut points),expected_results);
		assert_eq!(monotone_chain(&mut points),expected_results);
		assert_eq!(jarvis_march(&mut points),expected_results);
		
		
		let mut points = Vec::new();   
		points.push(Point2D::new(0.5,0.5));
		
		for _ in 0..100000{
			points.push( Point2D::new(rand::thread_rng().gen(),rand::thread_rng().gen()));
		}
		
		let graham   = graham_scan(&mut points);
		let monotone = monotone_chain(&mut points);
		let jarvis   = jarvis_march(&mut points);
		assert_eq!(graham,monotone);
		assert_eq!(graham,jarvis);
		
		
	}
	

}
