
#[allow(clippy::all)]
pub mod keyvalue{
  #[allow(unused_imports)]
  use wit_bindgen_guest_rust::rt::{alloc, vec::Vec, string::String};
  
  /// common keyvalue errors
  #[derive(Clone)]
  pub enum KeyvalueError{
    KeyNotFound(String),
  }
  impl core::fmt::Debug for KeyvalueError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
      match self {
        KeyvalueError::KeyNotFound(e) => {
          f.debug_tuple("KeyvalueError::KeyNotFound").field(e).finish()
        }
      }
    }
  }
  impl core::fmt::Display for KeyvalueError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
      write!(f, "{:?}", self)}
    }
    
    impl std::error::Error for KeyvalueError {}
    /// get the payload for a given key
    pub fn get(key: &str,) -> Result<Vec<u8>,KeyvalueError>{
      unsafe {
        let vec0 = key;
        let ptr0 = vec0.as_ptr() as i32;
        let len0 = vec0.len() as i32;
        
        #[repr(align(4))]
        struct RetArea([u8; 16]);
        let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
        let ptr1= ret_area.as_mut_ptr() as i32;
        
        #[link(wasm_import_module = "keyvalue")]
        extern "C" {
          #[cfg_attr(target_arch = "wasm32", link_name = "get")]
          #[cfg_attr(not(target_arch = "wasm32"), link_name = "keyvalue_get")]
          fn wit_import(
          _: i32, _: i32, _: i32, );
        }
        wit_import(ptr0, len0, ptr1);
        match i32::from(*((ptr1 + 0) as *const u8)) {
          0 => Ok({
            let len2 = *((ptr1 + 8) as *const i32) as usize;
            
            Vec::from_raw_parts(*((ptr1 + 4) as *const i32) as *mut _, len2, len2)
          }),
          1 => Err(match i32::from(*((ptr1 + 4) as *const u8)) {
            0 => KeyvalueError::KeyNotFound({
              let len3 = *((ptr1 + 12) as *const i32) as usize;
              
              String::from_utf8(Vec::from_raw_parts(*((ptr1 + 8) as *const i32) as *mut _, len3, len3)).unwrap()
            }),
            _ => panic!("invalid enum discriminant"),
          }),
          _ => panic!("invalid enum discriminant"),
        }
      }
    }
    /// set the payload for a given key
    pub fn set(key: &str,value: &[u8],) -> Result<(),KeyvalueError>{
      unsafe {
        let vec0 = key;
        let ptr0 = vec0.as_ptr() as i32;
        let len0 = vec0.len() as i32;
        let vec1 = value;
        let ptr1 = vec1.as_ptr() as i32;
        let len1 = vec1.len() as i32;
        
        #[repr(align(4))]
        struct RetArea([u8; 16]);
        let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
        let ptr2= ret_area.as_mut_ptr() as i32;
        
        #[link(wasm_import_module = "keyvalue")]
        extern "C" {
          #[cfg_attr(target_arch = "wasm32", link_name = "set")]
          #[cfg_attr(not(target_arch = "wasm32"), link_name = "keyvalue_set")]
          fn wit_import(
          _: i32, _: i32, _: i32, _: i32, _: i32, );
        }
        wit_import(ptr0, len0, ptr1, len1, ptr2);
        match i32::from(*((ptr2 + 0) as *const u8)) {
          0 => Ok(()),
          1 => Err(match i32::from(*((ptr2 + 4) as *const u8)) {
            0 => KeyvalueError::KeyNotFound({
              let len3 = *((ptr2 + 12) as *const i32) as usize;
              
              String::from_utf8(Vec::from_raw_parts(*((ptr2 + 8) as *const i32) as *mut _, len3, len3)).unwrap()
            }),
            _ => panic!("invalid enum discriminant"),
          }),
          _ => panic!("invalid enum discriminant"),
        }
      }
    }
    /// delete the payload for a given key
    pub fn delete(key: &str,) -> Result<(),KeyvalueError>{
      unsafe {
        let vec0 = key;
        let ptr0 = vec0.as_ptr() as i32;
        let len0 = vec0.len() as i32;
        
        #[repr(align(4))]
        struct RetArea([u8; 16]);
        let mut ret_area = core::mem::MaybeUninit::<RetArea>::uninit();
        let ptr1= ret_area.as_mut_ptr() as i32;
        
        #[link(wasm_import_module = "keyvalue")]
        extern "C" {
          #[cfg_attr(target_arch = "wasm32", link_name = "delete")]
          #[cfg_attr(not(target_arch = "wasm32"), link_name = "keyvalue_delete")]
          fn wit_import(
          _: i32, _: i32, _: i32, );
        }
        wit_import(ptr0, len0, ptr1);
        match i32::from(*((ptr1 + 0) as *const u8)) {
          0 => Ok(()),
          1 => Err(match i32::from(*((ptr1 + 4) as *const u8)) {
            0 => KeyvalueError::KeyNotFound({
              let len2 = *((ptr1 + 12) as *const i32) as usize;
              
              String::from_utf8(Vec::from_raw_parts(*((ptr1 + 8) as *const i32) as *mut _, len2, len2)).unwrap()
            }),
            _ => panic!("invalid enum discriminant"),
          }),
          _ => panic!("invalid enum discriminant"),
        }
      }
    }
    
  }
  
  
  #[allow(clippy::all)]
  pub mod results{
    #[allow(unused_imports)]
    use wit_bindgen_guest_rust::rt::{alloc, vec::Vec, string::String};
    
    pub trait Results {
      fn handler() -> String;
    }
    
    #[doc(hidden)]
    pub unsafe fn call_handler<T: Results>() -> i32 {
      let result0 = T::handler();
      let ptr1 = RET_AREA.0.as_mut_ptr() as i32;
      let vec2 = (result0.into_bytes()).into_boxed_slice();
      let ptr2 = vec2.as_ptr() as i32;
      let len2 = vec2.len() as i32;
      core::mem::forget(vec2);
      *((ptr1 + 4) as *mut i32) = len2;
      *((ptr1 + 0) as *mut i32) = ptr2;
      ptr1
    }
    
    #[doc(hidden)]
    pub unsafe fn post_return_handler<T: Results>(arg0: i32,) {
      wit_bindgen_guest_rust::rt::dealloc(*((arg0 + 0) as *const i32), (*((arg0 + 4) as *const i32)) as usize, 1);
    }
    
    #[repr(align(4))]
    struct ResultsRetArea([u8; 8]);
    static mut RET_AREA: ResultsRetArea = ResultsRetArea([0; 8]);
    
  }
  
  
  /// Declares the export of the component's world for the
  /// given type.
  
  macro_rules! export_results(($t:ident) => {
    const _: () = {
      
      #[doc(hidden)]
      #[export_name = "handler"]
      #[allow(non_snake_case)]
      unsafe extern "C" fn __export_exports_handler() -> i32 {
        results::call_handler::<$t>()
      }
      
      #[doc(hidden)]
      #[export_name = "cabi_post_handler"]
      #[allow(non_snake_case)]
      unsafe extern "C" fn __post_return_exports_handler(arg0: i32,) {
        results::post_return_handler::<$t>(arg0,)
      }
      
    };
    
    #[used]
    #[doc(hidden)]
    #[cfg(target_arch = "wasm32")]
    static __FORCE_SECTION_REF: fn() = __force_section_ref;
    #[doc(hidden)]
    #[cfg(target_arch = "wasm32")]
    fn __force_section_ref() {
      __link_section()
    }
  });
  
  #[cfg(target_arch = "wasm32")]
  #[link_section = "component-type:results"]
  pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 188] = [1, 0, 7, 114, 101, 115, 117, 108, 116, 115, 0, 97, 115, 109, 10, 0, 1, 0, 7, 138, 1, 9, 64, 0, 0, 115, 113, 1, 13, 107, 101, 121, 45, 110, 111, 116, 45, 102, 111, 117, 110, 100, 1, 115, 0, 112, 125, 106, 1, 2, 1, 1, 64, 1, 3, 107, 101, 121, 115, 0, 3, 106, 0, 1, 1, 64, 2, 3, 107, 101, 121, 115, 5, 118, 97, 108, 117, 101, 2, 0, 5, 64, 1, 3, 107, 101, 121, 115, 0, 5, 66, 8, 2, 3, 2, 1, 1, 4, 14, 107, 101, 121, 118, 97, 108, 117, 101, 45, 101, 114, 114, 111, 114, 0, 3, 0, 0, 2, 3, 2, 1, 4, 4, 3, 103, 101, 116, 0, 1, 1, 2, 3, 2, 1, 6, 4, 3, 115, 101, 116, 0, 1, 2, 2, 3, 2, 1, 7, 4, 6, 100, 101, 108, 101, 116, 101, 0, 1, 3, 10, 13, 1, 8, 107, 101, 121, 118, 97, 108, 117, 101, 0, 5, 8, 11, 12, 1, 7, 104, 97, 110, 100, 108, 101, 114, 0, 3, 0];
  
  #[inline(never)]
  #[doc(hidden)]
  #[cfg(target_arch = "wasm32")]
  pub fn __link_section() {}
  