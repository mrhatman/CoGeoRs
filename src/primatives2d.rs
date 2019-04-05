use num_traits::Float;
use num_traits::Zero;
use std::cmp::Ordering;

///2D Point
#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub struct Point2D<T: Float+Zero>{
	///Point's x position
	pub x : T,
	///Point's y position
	pub y : T,
}

impl<T: Float+Zero> Point2D<T >{

	///Creates a Point.
	pub fn new(x:T,y:T) -> Self{
		Point2D{x,y}
	}

	///Returns the rotational direction of the points (self,p1,p1).
	pub fn turn_direction(&self, p1: &Point2D<T>,p2: &Point2D<T>) -> TurnDirection{
		let line1 = (p1.x -self.x, p1.y -self.y);
		let line2 = (p2.x -self.x, p2.y -self.y);
		
		let det = (line1.0 * line2.1) - (line1.1 * line2.0);
		
		if det < T::zero()
		{	
			TurnDirection::RightTurn
		}
		else if det > T::zero()
		{	
			TurnDirection::LeftTurn
		}
		else{	
			TurnDirection::NoTurn
		}
	}

	///Partial comparision function ordered by x value then by y in case of ties.
	pub fn x_then_y_partial_cmp(&self,other: &Point2D<T>) -> Option<Ordering> {
		if self.x != other.x{
			self.x.partial_cmp(&other.x)
		}
		else{
			self.y.partial_cmp(&other.y)
		}
	}

	///Partial comparision function ordered polar cordinate in relation to rotational point.
	pub fn rotation_point_cmp(&self,other: &Point2D<T>,rotational_point: &Point2D<T>) -> Option<Ordering> {
		match rotational_point.turn_direction(self,other){
			TurnDirection::LeftTurn => Some(Ordering::Greater),
			TurnDirection::RightTurn  => Some(Ordering::Less),
			TurnDirection::NoTurn    => {
				self.x_then_y_partial_cmp(other)
			},
		}
	}
}

///2D Line
#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub struct Line2D<T: Float+Zero>{
	///Point 1
	pub p1 : Point2D<T>,
	///Point 2
	pub p2 : Point2D<T>,
}

impl<T: Float+Zero> Line2D<T >{
	///Creates a 2D Line from p1 to p2.
	pub fn new(p1:Point2D<T>,p2:Point2D<T>) -> Self{
		Line2D{p1,p2}
	}
	
	///returns true if 'point' is on this line
	pub fn contains_point(&self, point: &Point2D<T>) -> bool{
		
		if point.x < self.p1.x.min(self.p2.x) ||
			(point.y < self.p1.x.min(self.p2.y)) ||
			(point.x > self.p1.x.max(self.p2.x)) ||
			(point.y > self.p1.y.max(self.p2.y)){
				
				return false;
		}
		
		let range_x = self.p2.x - self.p1.x;
		let range_y = self.p2.y - self.p1.y;
	
		let x = (point.x - self.p1.x) / range_x; 
		let y = (point.y - self.p1.y) / range_y;

		x == y
	}	

	///Returns true if the lines intersect.
	pub fn intersects_with_line(&self, other: &Line2D<T>) -> bool{
		
		((self.p1.turn_direction(&self.p2,&other.p1) !=  self.p1.turn_direction(&self.p2,&other.p2)) &&
		(other.p1.turn_direction(&other.p2,&self.p1) != other.p1.turn_direction(&other.p2,&self.p2))) ||
		((self.p1.turn_direction(&self.p2,&other.p1) == TurnDirection::NoTurn) && (self.p1.turn_direction(&self.p2,&other.p2) == TurnDirection::NoTurn)&&
		(self.contains_point(&other.p1) ||self.contains_point(&other.p2)))

	}	

	///Returns the intersection point between 2 lines, or None if they don't intersect.
	pub fn intersection_point(&self, other: &Line2D<T>) -> Option<Point2D<T>>{
		
		if !self.intersects_with_line(other){
			None
		}
		else{
			let slope_self = (self.p2.y - self.p1.y)/(self.p2.x - self.p1.x);
			let slope_other = (other.p2.y - other.p1.y)/(other.p2.x - other.p1.x);
			
			
			if slope_other == slope_self{
				if self.p1.y < other.p1.y.max(other.p2.y) && self.p1.y > other.p1.y.min(other.p2.y) {
					Some(self.p1)
				}
				else{
					Some(self.p2)
				}
			}
			else{
				let x = (other.p1.y - self.p1.y - (slope_other*other.p1.x) + (slope_self*self.p1.x))/ (slope_self-slope_other);
				let y = slope_self*(x-self.p1.x) + self.p1.y;
				
				Some(Point2D::new(x,y))
			}
		}
		
	}
	
}

///Enum representing rotation.
#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub enum TurnDirection{
	RightTurn,
	LeftTurn,
	NoTurn,
}

#[cfg(test)]
mod primatives_test {
	use super::*;
	
    #[test]
    fn point_new_test() {
        let p =Point2D::new(2.3,6.7);
		assert_eq!(p.x, 2.3);
		assert_eq!(p.y, 6.7);
	}    
	
	#[test]
    fn turn_direction_test() {
        let p =Point2D::new(0.0,0.0);
		assert_eq!(p.turn_direction(&Point2D::new(1.0,1.0),&Point2D::new(1.0,0.0)), TurnDirection::RightTurn);
		assert_eq!(p.turn_direction(&Point2D::new(1.0,1.0),&Point2D::new(1.0,2.0)), TurnDirection::LeftTurn);
		assert_eq!(p.turn_direction(&Point2D::new(1.0,1.0),&Point2D::new(2.0,2.0)), TurnDirection::NoTurn);

	}	
	
	#[test]
    fn line_intersection_test() {
        let line1 = Line2D::new(Point2D::new(0.0,0.0),Point2D::new(1.0,1.0));
        let line2 = Line2D::new(Point2D::new(1.0,0.0),Point2D::new(0.0,1.0));
        let line3 = Line2D::new(Point2D::new(2.0,0.0),Point2D::new(0.0,2.0));
        let line4 = Line2D::new(Point2D::new(0.5,0.5),Point2D::new(1.5,1.5));
        let line5 = Line2D::new(Point2D::new(4.5,4.5),Point2D::new(1.5,1.5));
		assert!(line1.intersects_with_line(&line1));
		assert!(line1.intersects_with_line(&line2));
		assert!(line1.intersects_with_line(&line3));
		assert!(!line3.intersects_with_line(&line2));
		assert!(!line2.intersects_with_line(&line3));
		assert!(line1.intersects_with_line(&line4));
		assert!(!line1.intersects_with_line(&line5));
		
		assert_eq!(line1.intersection_point(&line2),Some(Point2D::new(0.5,0.5)) );
		assert_eq!(line2.intersection_point(&line1),Some(Point2D::new(0.5,0.5)) );
		assert_eq!(line1.intersection_point(&line3),Some(Point2D::new(1.0,1.0)) );
		assert_eq!(line3.intersection_point(&line2),None );
		assert_eq!(line2.intersection_point(&line3),None );
		assert_eq!(line1.intersection_point(&line4),Some(Point2D::new(1.0,1.0)) );
		assert_eq!(line1.intersection_point(&line5),None );


	}	
	
	#[test]
    fn line_contains_point_test() {
        let line1 = Line2D::new(Point2D::new(0.0,0.0),Point2D::new(1.0,1.0));
        let line2 = Line2D::new(Point2D::new(1.0,1.0),Point2D::new(0.0,0.0));
		assert!(line1.contains_point(&Point2D::new(0.5,0.5)));
		assert!(line1.contains_point(&Point2D::new(0.0,0.0)));
		assert!(!line1.contains_point(&Point2D::new(0.5,0.4)));
		assert!(!line1.contains_point(&Point2D::new(1.5,1.5)));
		assert!(line2.contains_point(&Point2D::new(0.5,0.5)));
		assert!(line2.contains_point(&Point2D::new(0.0,0.0)));
		assert!(!line2.contains_point(&Point2D::new(0.5,0.4)));
		assert!(!line2.contains_point(&Point2D::new(1.5,1.5)));

	}
}
