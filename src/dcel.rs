use crate::primatives2d::{Point2D};
use crate::algorithms::polygon_area::shoelace;
use num_traits::Float;
use std::fmt;

use std::rc::Rc;
use std::cell::RefCell;

type Ptr<T> = Rc<RefCell<T>>;



///Doublely Connected Edge List.
pub struct DCEL<T:Float>{
    vertices   : Vec<Ptr<Vertex<T>>>,
    faces      : Vec<Ptr<Face<T>>>,
    half_edges : Vec<Ptr<HalfEdge<T>>>,
	vertex_count : usize,
	face_count   : usize,
	half_edge_count :usize,
}

impl<T :Float> DCEL<T>{


	fn create_empty() -> Self{
		DCEL{vertices:Vec::new(),faces: Vec::new(),half_edges: Vec::new(),vertex_count:0,face_count:0,half_edge_count:0}
	}



	fn create_from_point_list(points : &[Point2D<T>]) -> Self{
		let mut dcel = DCEL::create_empty();

		let outer_face = dcel.create_face();
		let inner_face = dcel.create_face();

		let start_vertex = dcel.create_vertex(points[0]);
		let (out_edge,in_edge) = dcel.create_twin_edges();
		out_edge.borrow_mut().incident_face = Some(outer_face.clone());
		in_edge.borrow_mut().incident_face  = Some(inner_face.clone());

		out_edge.borrow_mut().origin = Some(start_vertex.clone());
		start_vertex.borrow_mut().incident_edge = Some(out_edge.clone());

		outer_face.borrow_mut().inner_component.push( out_edge.clone());
		inner_face.borrow_mut().outer_component = Some( in_edge.clone());

		let ( mut last_outer_edge, mut last_inner_edge) =(out_edge.clone(),in_edge.clone());

		for q in 1.. points.len(){
			let (out_edge,in_edge) = dcel.create_twin_edges();

			out_edge.borrow_mut().incident_face = Some(outer_face.clone());
			in_edge.borrow_mut().incident_face  = Some(inner_face.clone());

			let vertex = dcel.create_vertex(points[q]);
			out_edge.borrow_mut().origin = Some(vertex.clone());
			vertex.borrow_mut().incident_edge = Some(out_edge.clone());


			last_inner_edge.borrow_mut().origin = Some(vertex.clone());
			last_inner_edge.borrow_mut().prev = Some(in_edge.clone());
			last_outer_edge.borrow_mut().next = Some(out_edge.clone());
			out_edge.borrow_mut().prev  = Some(last_outer_edge.clone());
			in_edge.borrow_mut().next = Some(last_inner_edge.clone());



			last_outer_edge =out_edge;
			last_inner_edge =in_edge;
		}

		last_inner_edge.borrow_mut().origin =Some(start_vertex.clone());
		last_inner_edge.borrow_mut().prev = Some(in_edge.clone());
		last_outer_edge.borrow_mut().next = Some(out_edge.clone());
		out_edge.borrow_mut().prev  = Some(last_outer_edge.clone());
		in_edge.borrow_mut().next = Some(last_inner_edge.clone());


		dcel
	}

	fn verify(&self) -> Result<bool,String>{

		for edge in &self.half_edges{

			//Twin Test
			if let Some(twin) = &edge.borrow().twin{
				if twin.borrow().twin != Some(edge.clone()) {
					return Err(format!("Edge {} has twin {}, but its twin is not {}",edge.borrow().index,twin.borrow().index, edge.borrow().index ));
				}
			}
			else{
				return Err(format!("Edge {} does not have a twin", edge.borrow().index));
			}


			//Next Test
			if let Some(next) = &edge.borrow().next{
				if next.borrow().prev != Some(edge.clone()) {
					return Err(format!("Edge {} has next {}, but its prev is not {}",edge.borrow().index,next.borrow().index, edge.borrow().index ));
				}
			}
			else{
				return Err(format!("Edge {} does not have a next", edge.borrow().index));
			}


			//Prev Test
			if let Some(prev) = &edge.borrow().prev{
				if prev.borrow().next != Some(edge.clone()) {
					return Err(format!("Edge {} has prev {}, but its next is not {}",edge.borrow().index,prev.borrow().index, edge.borrow().index ));
				}
			}
			else{
				return Err(format!("Edge {} does not have a prev", edge.borrow().index));
			}

			//Verify origin and incident_face exist
			if edge.borrow().origin.is_none(){
				return Err(format!("Edge {} does not have an origin", edge.borrow().index));
			}
			if edge.borrow().incident_face.is_none(){
				return Err(format!("Edge {} does not have an incident_face", edge.borrow().index));
			}

		}


		for vertex in &self.vertices{
			//Verify incident_edge exist
			if let Some(edge) = &vertex.borrow().incident_edge {
				if edge.borrow().origin != Some(vertex.clone()) {
					return Err(format!("Vertex {} has incident_edge {}, but it's origin is not {}",vertex.borrow().index,edge.borrow().index, vertex.borrow().index) );
				}
			}
			else{
				return Err(format!("Edge {} does not have an incident_edge", vertex.borrow().index));
			}
		}

		let mut polygons_to_check =Vec::new();

		for face in &self.faces{
			//Verify incident_edge exist
			if let Some(face_edge) = &face.borrow().outer_component {
				polygons_to_check.push((face_edge.clone(),face));
			}

			for inner_edge in &face.borrow().inner_component{
				polygons_to_check.push((inner_edge.clone(),face));
			}

		}

		for (starting_edge,face) in  polygons_to_check{

			for edge in PolygonIterator::new(starting_edge.clone()){
				if edge.borrow().incident_face != Some(face.clone()){
					return Err(format!("Edge {} on the polygon starting at {} incident face is not {}", edge.borrow().index ,starting_edge.borrow().index,face.borrow().index));
				}
			}

		}

		Ok(true)
	}



	///inserts a edge from the origin of half_edge1 to the orgin of half_edge2. half_edge2 get a new face.
	///Does not check if there is any intersections. All inner_component are keep with original face.
	///The twin pair of new half edges and the new face are returned.
	fn unchecked_divide_face(&mut self, half_edge1: Ptr<HalfEdge<T>>, half_edge2: Ptr<HalfEdge<T>>) -> (Ptr<HalfEdge<T>>,Ptr<HalfEdge<T>>,Ptr<Face<T>>){
		let half_edge1_prev = half_edge1.borrow().prev.as_ref().unwrap().clone();
		let half_edge2_prev = half_edge2.borrow().prev.as_ref().unwrap().clone();
		let (old_face_edge,new_face_edge) = self.create_twin_edges();
		let new_face = self.create_face();
		let old_face = half_edge1.borrow().incident_face.as_ref().unwrap().clone();
		new_face.borrow_mut().outer_component = Some(new_face_edge.clone());
		old_face.borrow_mut().outer_component = Some(old_face_edge.clone());

		half_edge1.borrow_mut().prev = Some(old_face_edge.clone());
		old_face_edge.borrow_mut().next = Some(half_edge1.clone());

		half_edge2.borrow_mut().prev = Some(new_face_edge.clone());
		new_face_edge.borrow_mut().next = Some(half_edge2.clone());

		half_edge1_prev.borrow_mut().next = Some(new_face_edge.clone());
		new_face_edge.borrow_mut().prev = Some(half_edge1_prev.clone());

		half_edge2_prev.borrow_mut().next = Some(old_face_edge.clone());
		old_face_edge.borrow_mut().prev = Some(half_edge2_prev.clone());

		old_face_edge.borrow_mut().origin = half_edge2.borrow().origin.clone();
		new_face_edge.borrow_mut().origin =  half_edge1.borrow().origin.clone();

		old_face_edge.borrow_mut().incident_face = Some(old_face.clone());
		new_face_edge.borrow_mut().incident_face = Some(new_face.clone());


		for edge in PolygonIterator::new(new_face_edge.clone()){
			edge.borrow_mut().incident_face = Some(new_face.clone());
		}

		(old_face_edge,new_face_edge,new_face)
	}


	fn create_vertex(&mut self,point:Point2D<T>) -> Ptr<Vertex<T>>{

		let index =self.vertex_count;
		self.vertex_count += 1;
		let vertex = Vertex{index , coordinate: point, incident_edge: None};
		let vertex_ptr = Rc::new(RefCell::new(vertex));
		self.vertices.push(vertex_ptr.clone());
		vertex_ptr
	}


	fn create_face(&mut self) -> Ptr<Face<T>>{

		let index =self.face_count;
		self.face_count += 1;
		let face = Face{index ,outer_component: None, inner_component:Vec::new()};
		let face_ptr = Rc::new(RefCell::new(face));
		self.faces.push(face_ptr.clone());
		face_ptr

	}


	fn create_twin_edges(&mut self) -> (Ptr<HalfEdge<T>>,Ptr<HalfEdge<T>>){

		let index =self.half_edge_count;
		self.half_edge_count += 2;
		let twin1 = HalfEdge{index,origin: None,twin:None,next:None,prev:None,incident_face:None};
		let twin2 = HalfEdge{index:index+1,origin: None,twin:None,next:None,prev:None,incident_face:None};
		let twin1_ptr = Rc::new(RefCell::new(twin1));
		let twin2_ptr = Rc::new(RefCell::new(twin2));


		twin1_ptr.borrow_mut().twin = Some(twin2_ptr.clone());
		twin2_ptr.borrow_mut().twin = Some(twin1_ptr.clone());

		self.half_edges.push(twin1_ptr.clone());
		self.half_edges.push(twin2_ptr.clone());
		(twin1_ptr,twin2_ptr)
	}

	/*
	fn to_string(&self) -> String {
        let mut str = String::new();

		/*
		str += &format!("Vertex coordinate Incident-Edge\n");
		let mut count =0;
		for vert in self.vertices{
			str += &format!("v{} ({},{}) e{}\n",count,vert.coordinate.x,vert.coordinate.y,vert.incident_edge.unwrap());
			count += 1;
		}

		count =0;
		str += &format!("Face Outer-Component Inner-Component\n");
		for face in self.faces{
			str += &format!("f{} \n",count);
			count += 1;
		}

		*/
		str
    }
	*/



	///Face must have a outer_component
	///Finds area of a polygon including the inner_component
	fn get_polygon_area_inclusive(&self, face:  Ptr<Face<T >>) -> T{

		shoelace((PolygonIterator::new(face.borrow().outer_component.as_ref().unwrap().clone())
			.map(|edge| Box::new(edge.borrow().origin.as_ref().unwrap().borrow().coordinate)) )
			)

	}




	///Face must have a outer_component
	fn get_polygon_points_from_face(&self, face: Ptr<Face<T>>) -> Vec<Point2D<T>>{
		let mut point_list = Vec::new();

		for edge in PolygonIterator::new(face.borrow().outer_component.as_ref().unwrap().clone()){
			point_list.push(edge.borrow().origin.as_ref().unwrap().borrow().coordinate );
		}
		point_list
	}



	///
	fn get_polygons(&self) -> Vec<Vec<Point2D<T>>>{
		let mut polygon_list = Vec::new();

		for face in &self.faces{
			if face.borrow().outer_component.is_some(){
				polygon_list.push(self.get_polygon_points_from_face(face.clone()));
			}
		}
		polygon_list
	}

}


struct Vertex<T:Float>{
	index		  : usize,
    coordinate    : Point2D<T>,
    incident_edge : Option<Ptr<HalfEdge<T>>>,
}
impl<T:Float> PartialEq for Vertex<T> {
    fn eq(&self, other: &Vertex<T>) -> bool {
		self.index == other.index
	}
}

struct Face<T:Float>{
	index		   : usize,
    outer_component: Option<Ptr<HalfEdge<T>>>,
    inner_component: Vec<Ptr<HalfEdge<T>>>,

}
impl<T:Float> PartialEq for Face<T> {
    fn eq(&self, other: &Face<T>) -> bool {
		self.index == other.index
	}
}

struct HalfEdge<T:Float>{
	index			: usize,
    origin          : Option<Ptr<Vertex<T>>>,
    twin            : Option<Ptr<HalfEdge<T>>>,
    next            : Option<Ptr<HalfEdge<T>>>,
    prev            : Option<Ptr<HalfEdge<T>>>,
    incident_face   : Option<Ptr<Face<T>>>,

}

impl<T:Float> HalfEdge<T>{
	fn clean(&mut self){
		self.origin =None;
		self.twin =None;
		self.next =None;
		self.prev =None;
		self.incident_face =None;
	}
}

impl<T:Float> PartialEq for HalfEdge<T> {
    fn eq(&self, other: &HalfEdge<T>) -> bool {
		self.index == other.index
	}
}

struct PolygonIterator<T: Float>{
	starting_edge : Ptr<HalfEdge<T>>,
	current_edge  : Ptr<HalfEdge<T>>,
	finished : bool
}

impl<T: Float> PolygonIterator<T>{
	fn new(starting_edge: Ptr<HalfEdge<T>>) -> Self{
		PolygonIterator{current_edge : starting_edge.clone(),starting_edge,finished: false}
	}

}

impl<T:Float> Iterator for PolygonIterator<T> {
    type Item = Ptr<HalfEdge<T>>;
    fn next(&mut self) -> Option<Self::Item> {


		if !self.finished{
			self.current_edge = self.current_edge.clone().borrow().next.as_ref().unwrap().clone();

			if self.current_edge == self.starting_edge {
				self.finished = true
			}

			Some( self.current_edge.clone())

		}
		else{
			None
		}


    }
}





#[cfg(test)]
mod algorithms_test {
	use super::*;

    #[test]
    fn create_from_point_list_test() {
        let points = vec!(
			Point2D::new(0.0,0.0),
			Point2D::new(0.0,1.0),
			Point2D::new(1.0,1.0),
			Point2D::new(1.0,0.0)
		);
		let dcel = DCEL::create_from_point_list(&points);
		assert!(dcel.verify().unwrap());
	}


	#[test]
    fn unchecked_divide_face_test() {
        let points = vec!(
			Point2D::new(0.0,0.0),
			Point2D::new(0.0,1.0),
			Point2D::new(1.0,1.0),
			Point2D::new(1.0,0.0)
		);
		let mut dcel = DCEL::create_from_point_list(&points);
		assert!(dcel.verify().unwrap());
		dcel.unchecked_divide_face(dcel.half_edges[1].clone(),dcel.half_edges[5].clone());
		assert!(dcel.verify().unwrap());
		assert_eq!(dcel.faces.len(),3);
	}

	#[test]
    fn get_polygon_area_inclusive_test() {
		let points = vec!(
			Point2D::new(0.0,0.0),
			Point2D::new(0.0,1.0),
			Point2D::new(1.0,1.0),
			Point2D::new(1.0,0.0)
		);
		let mut dcel = DCEL::create_from_point_list(&points);
		assert_eq!(dcel.get_polygon_area_inclusive(dcel.faces[1].clone()), 1.0 );
	}



}
