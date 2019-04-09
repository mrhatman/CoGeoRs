use crate::primatives2d::{Point2D};
use num_traits::Float;
use std::fmt;


type VertexIndex   = usize;
type FaceIndex     = usize;
type HalfEdgeIndex = usize;

///Doublely Connected Edge List.
pub struct DCEL<T:Float>{
    vertices   : Vec<Vertex<T>>,
    faces      : Vec<Face>,
    half_edges : Vec<HalfEdge>,
}

impl<T :Float> DCEL<T>{
	
	fn create_empty() -> Self{
		DCEL{vertices:Vec::new(),faces: Vec::new(),half_edges: Vec::new()}
	}
	
	fn create_from_point_list(points : &[Point2D<T>]) -> Self{
		let mut dcel = DCEL::create_empty();
		
		let outer_face = dcel.create_face();
		let inner_face = dcel.create_face();
		
		dcel.create_vertices(points[0]);
		let (out_edge,in_edge) = dcel.create_twin_edges();
		dcel.half_edges[out_edge].incident_face = Some(outer_face);
		dcel.half_edges[in_edge].incident_face  = Some(inner_face);
		
		dcel.half_edges[out_edge].origin = Some(0);
		dcel.vertices[0].incident_edge = Some(out_edge);
		
		dcel.faces[outer_face].inner_component.push( out_edge);
		dcel.faces[inner_face].outer_component = Some( in_edge);
		
		let ( mut last_outer_edge, mut last_inner_edge) 
			=(out_edge,in_edge);
		
		for q in 1.. points.len(){
			let (out_edge,in_edge) = dcel.create_twin_edges();
			
			dcel.half_edges[out_edge].incident_face = Some(outer_face);
			dcel.half_edges[in_edge].incident_face  = Some(inner_face);
			
			let vert = dcel.create_vertices(points[q]);
			dcel.half_edges[out_edge].origin = Some(vert);
			dcel.vertices[vert].incident_edge = Some(out_edge);
			
			
			dcel.half_edges[last_inner_edge].origin = Some(q);
			dcel.half_edges[last_inner_edge].prev = Some(in_edge);
			dcel.half_edges[last_outer_edge].next = Some(out_edge);
			dcel.half_edges[out_edge].prev  = Some(last_outer_edge);
			dcel.half_edges[in_edge].next = Some(last_inner_edge);
			
			
			
			last_outer_edge =out_edge;
			last_inner_edge =in_edge;	
		}
		
		dcel.half_edges[last_inner_edge].origin =Some(0);
		dcel.half_edges[last_inner_edge].prev = Some(in_edge);
		dcel.half_edges[last_outer_edge].next = Some(out_edge);
		dcel.half_edges[out_edge].prev  = Some(last_outer_edge);
		dcel.half_edges[in_edge].next = Some(last_inner_edge);
		
		
		dcel
	}
	
	fn verify(&self) -> Result<bool,String>{
		
		for q in 0..self.half_edges.len(){
			
			//Twin Test
			if let Some(twin_index) = self.half_edges[q].twin{
				if self.half_edges[twin_index].twin != Some(q) {
					return Err(format!("Edge {} has twin {}, but it's twin is {:?}",q,twin_index, self.half_edges[q].twin) );
				}
			}
			else{
				return Err(format!("Edge {} does not have a twin", q));
			}			
			//Next Test
			if let Some(next_index) = self.half_edges[q].next{
				if self.half_edges[next_index].prev != Some(q) {
					return Err(format!("Edge {} has next {}, but it's prev is {:?}",q,next_index, self.half_edges[q].twin) );
				}
			}
			else{
				return Err(format!("Edge {} does not have a next", q));
			}
			//Prev Test
			if let Some(prev_index) = self.half_edges[q].prev{
				if self.half_edges[prev_index].next != Some(q) {
					return Err(format!("Edge {} has next {}, but it's prev is {:?}",q,prev_index, self.half_edges[q].twin) );
				}
			}
			else{
				return Err(format!("Edge {} does not have a prev", q));
			}
			
			//Verify origin and incident_face exist
			if self.half_edges[q].origin.is_none(){
				return Err(format!("Edge {} does not have an origin", q));
			}			
			if self.half_edges[q].incident_face.is_none(){
				return Err(format!("Edge {} does not have an incident_face", q));
			}
			
		}
		for q in 0..self.vertices.len(){
			//Verify incident_edge exist
			if let Some(edge_index) = self.vertices[q].incident_edge {
				if self.half_edges[edge_index].origin != Some(q) {
					return Err(format!("Vertex {} has incident_edge {}, but it's origin is {:?}",q,edge_index, self.half_edges[edge_index].origin) );
				}
			}
			else{
				return Err(format!("Edge {} does not have an incident_edge", q));
			}
		}

		let mut polygons_to_check =Vec::new();

		for q in 0..self.faces.len(){
			//Verify incident_edge exist
			if let Some(face_edge_index) = self.faces[q].outer_component {
				polygons_to_check.push((q,face_edge_index));
			}

			for &inner_edge in &self.faces[q].inner_component{
				polygons_to_check.push((q,inner_edge));
			}

		}

		for (starting_edge,face) in  polygons_to_check{
			let mut current_edge = starting_edge;
			loop{
				if self.half_edges[current_edge].incident_face.unwrap() != face{
					return Err(format!("Edge {} on the polygon starting at {} incident face is not {}", current_edge,starting_edge,face));
				}
				current_edge = self.half_edges[current_edge].next.unwrap();
				if current_edge != starting_edge {break}
			}
		}

		
		Ok(true)
	}
	
	fn create_vertices(&mut self,point:Point2D<T>) -> FaceIndex{
		let len = self.vertices.len();
		self.vertices.push(Vertex{coordinates: point, incident_edge: None});
		len
	}
	
	fn create_face(&mut self) -> FaceIndex{
		let len = self.faces.len();
		self.faces.push(Face{outer_component: None, inner_component:Vec::new()});
		len
	}
	
	fn create_twin_edges(&mut self) -> (HalfEdgeIndex,HalfEdgeIndex){
		let len = self.half_edges.len();
		
		let t1 = HalfEdge{origin: None,twin:Some(len+1),next:None,prev:None,incident_face:None};
		let t2 = HalfEdge{origin: None,twin:Some(len)  ,next:None,prev:None,incident_face:None};
		
		self.half_edges.push(t1);
		self.half_edges.push(t2);
		
		(len,len+1)
	}
	
	fn to_string(&self) -> String {
        let mut str = String::new();

		/*
		str += &format!("Vertex Coordinates Incident-Edge\n");
		let mut count =0;
		for vert in self.vertices{
			str += &format!("v{} ({},{}) e{}\n",count,vert.coordinates.x,vert.coordinates.y,vert.incident_edge.unwrap());
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
}

impl<T :Float> fmt::Display for DCEL<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",self.to_string())
    }
}

struct Vertex<T:Float>{
    coordinates   : Point2D<T>,
    incident_edge : Option<HalfEdgeIndex>,
}
struct Face{
    outer_component: Option<HalfEdgeIndex>,
    inner_component: Vec<HalfEdgeIndex>,

}
struct HalfEdge{
    origin          : Option<VertexIndex>,
    twin            : Option<HalfEdgeIndex>,
    next            : Option<HalfEdgeIndex>,
    prev            : Option<HalfEdgeIndex>,
    incident_face   : Option<FaceIndex>,

}

#[cfg(test)]
mod algorithms_test {
	use super::*;
	
    #[test]
    fn create_from_point_list_test() {
        let points = vec!(
			Point2D::new(0.0,0.0),
			Point2D::new(0.0,1.0),
			Point2D::new(1.0,0.0),
			Point2D::new(1.0,1.0),
			Point2D::new(0.5,0.5)
		);        
   
		
		assert!(DCEL::create_from_point_list(&points).verify().unwrap());
	}    

	

}
