//!Algorithms for calculating intersection points of line sets.

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use crate::primatives2d::{Point2D,Line2D};
use num_traits::{Float,Zero};

///Preforms the Bentley Ottmann Line Intersection Algorithm on a set of lines.
pub fn bentley_ottmann<T:Float+Zero>(lines : &[Line2D<T>]) -> Vec<Point2D<T>>  {
	//https://en.wikipedia.org/wiki/Bentley%E2%80%93Ottmann_algorithm

	let mut heap = BinaryHeap::new();
	let mut tree = Vec::new();
	let mut result = Vec::new();

	for &l in lines{
		heap.push(BentleyOttmannEvent{event_point: l.p1,event_type: BentleyOttmannEventType::LeftPointEvent(l) });

	}

	while let Some(event) = heap.pop(){
		match  event.event_type {
			BentleyOttmannEventType::LeftPointEvent(line) =>{
				let pos = tree.binary_search(&line).unwrap_or_else(|e| e);
				tree.insert(pos, line);
				if pos != 0 {
					if let Some(&left_line) = tree.get(pos-1) {
						if let Some(p) = line.intersection_point(&left_line) {
							heap.push(BentleyOttmannEvent { event_point: p, event_type: BentleyOttmannEventType::CrossLineEvent(left_line, line) });
						}
					}
				}
				if let Some(&right_line) = tree.get(pos+1){
					if let Some(p) = line.intersection_point(&right_line){
						heap.push(BentleyOttmannEvent{event_point: p,event_type: BentleyOttmannEventType::CrossLineEvent(line,right_line)});

					}
				}

			},
			BentleyOttmannEventType::RightPointEvent(line) =>{
				let pos = tree.binary_search(&line).unwrap_or_else(|e| e);
				tree.remove(pos);
				if let Some(&left_line) = tree.get(pos-1){
					if let Some(&right_line) = tree.get(pos){
						if let Some(p) = line.intersection_point(&left_line){
							heap.push(BentleyOttmannEvent{event_point: p,event_type: BentleyOttmannEventType::CrossLineEvent(left_line,right_line)});

						}
					}
				}
			},
			BentleyOttmannEventType::CrossLineEvent(left_line,right_line) =>{
				result.push(event.event_point);

			},

		}
	}


	result
}



struct  BentleyOttmannEvent<T: Float+Zero>{

	event_point :  Point2D<T>,
	event_type : BentleyOttmannEventType<T>
}
impl<T:Float+Zero> Ord for BentleyOttmannEvent<T> {
    fn cmp(&self, other: &BentleyOttmannEvent<T>) -> Ordering {
       self.partial_cmp(other).unwrap()
    }
}

impl<T:Float+Zero> PartialOrd for BentleyOttmannEvent<T> {
    fn partial_cmp(&self, other: &BentleyOttmannEvent<T>) -> Option<Ordering> {
         self.event_point.x_then_y_partial_cmp(&other.event_point )
    }
}

impl<T:Float+Zero> PartialEq for BentleyOttmannEvent<T> {
    fn eq(&self, other: &BentleyOttmannEvent<T>) -> bool {
        self.event_point == other.event_point &&
			self.event_type == other.event_type
    }
}
impl<T:Float+Zero> Eq for BentleyOttmannEvent<T> {
}

#[derive(Eq,PartialEq )]
enum BentleyOttmannEventType<T: Float+Zero> {
	LeftPointEvent(Line2D<T>),
	RightPointEvent(Line2D<T>),
	CrossLineEvent(Line2D<T>,Line2D<T>),
}


#[cfg(test)]
mod algorithms_test {
	use super::*;
	
    #[test]
    fn bentley_ottmann_test() {
        let lines = vec!(
			Line2D::new(Point2D::new(0.0,0.0),Point2D::new(2.0,2.0)),
			Line2D::new(Point2D::new(2.0,0.0),Point2D::new(0.0,2.0)),
		);
		let expected_results = vec!(
			Point2D::new(1.0,1.0),
		);    

		let result = bentley_ottmann(&lines);
		assert_eq!(result,expected_results);
	}    

}
