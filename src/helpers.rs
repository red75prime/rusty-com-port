use winapi::*;
use super::*; 
//use std::ptr;
use std::mem;

macro_rules! impl_into_iterator {
    ($factory:ident, $iterator:ident) => {
        impl<'a> IntoIterator for &'a $factory {
            type Item = <$iterator<'a, $factory> as Iterator>::Item;
            type IntoIter = $iterator<'a, $factory>;
            
            fn into_iter(self) -> Self::IntoIter {
                $iterator {
                    object: self,
                    cur_num: 0,
                }
            }
        }
    }
}

pub struct DXGIAdapterIterator<'a, T: 'a + TDXGIFactory1> {
    object: &'a T,
    cur_num: u32,
}

impl<'a, T: 'a + TDXGIFactory1> Iterator for DXGIAdapterIterator<'a, T> {
    type Item = (u32, DXGIAdapter1);
    
    fn next(&mut self) -> Option<Self::Item> {
        match self.object.enum_adapters1(self.cur_num) {
            Ok(adapter) => {
                let num1 = self.cur_num + 1;
                let num = mem::replace(&mut self.cur_num, num1);
                Some((num, adapter))
            },
            Err(err) if err == DXGI_ERROR_NOT_FOUND => {
                None
            },
            Err(err) => {
                panic!("Unexpected error while enumerating graphics adapters: 0x{:x}", err)
            }
        }
    }
}

impl_into_iterator!(DXGIFactory1, DXGIAdapterIterator);
impl_into_iterator!(DXGIFactory2, DXGIAdapterIterator);
impl_into_iterator!(DXGIFactory3, DXGIAdapterIterator);
impl_into_iterator!(DXGIFactory4, DXGIAdapterIterator);

pub struct DXGIOutputIterator<'a, T: 'a + TDXGIAdapter> {
    object: &'a T,
    cur_num: u32,
}

impl<'a, T: 'a + TDXGIAdapter> Iterator for DXGIOutputIterator<'a, T> {
    type Item = (u32, DXGIOutput);
    
    fn next(&mut self) -> Option<Self::Item> {
        match self.object.enum_outputs(self.cur_num) {
            Ok(output) => {
                let num1 = self.cur_num + 1;
                let num = mem::replace(&mut self.cur_num, num1);
                Some((num, output))
            },
            Err(err) if err == DXGI_ERROR_NOT_FOUND => {
                None
            },
            Err(err) => {
                panic!("Unexpected error while enumerating outputs: 0x{:x}", err)
            }
        }
    }
}

impl_into_iterator!(DXGIAdapter, DXGIOutputIterator);
impl_into_iterator!(DXGIAdapter1, DXGIOutputIterator);
impl_into_iterator!(DXGIAdapter2, DXGIOutputIterator);
impl_into_iterator!(DXGIAdapter3, DXGIOutputIterator);
