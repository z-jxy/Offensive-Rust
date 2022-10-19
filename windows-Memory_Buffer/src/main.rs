use windows::{
    core::*, 
    Win32::System::WinRT::IMemoryBufferByteAccess,
    Foundation::*,
};

unsafe fn as_mut_slice(buffer: &IMemoryBufferReference) -> Result<&mut [u8]> {
    let interop = buffer.cast::<IMemoryBufferByteAccess>()?;
    let mut data = std::ptr::null_mut();
    let mut len = 0;

    interop.GetBuffer(&mut data, &mut len)?;
    println!("\nACCESSING ALLOCATED BUFFER...\nVALUE: {:#?} BYTES\n", len );

    // Debug
    //Ok(std::slice::from_raw_parts_mut(data, len as _))

    Ok(std::slice::from_raw_parts_mut(data, len as _))
}

fn sys_info() -> Result<()> {
    let buffer = MemoryBuffer::Create(11)?;
    let refer = buffer.CreateReference()?;
    assert_eq!(refer.Capacity()?, 11);
    println!("Allocated memory with reference to address@:\n{:#?}", &refer );

    let my_msg: &[u8] = b"hello world";
    
    {
        let slice = unsafe { as_mut_slice(&refer)? };
        slice.copy_from_slice(my_msg);
        
        println!("[+]SUCCESS\n\nWrote: {:?}\ninto address@:\n{:#?}", my_msg, &refer);
    }

    {
        let slice = unsafe { as_mut_slice(&refer)? };
        assert_eq!(slice, b"hello world");
        println!("[+]SUCCESS\n\nRead: {:?}\nfrom address@:\n{:#?}", my_msg, &refer);
        println!("\nMessage translated from UTF-8:\n\n{:?}", std::str::from_utf8(slice));
    }

    println!("\nDone!\n" );
    Ok(())
}

fn main() -> Result<()> {
    println!("Starting intial buffer alloc!\n" );
    sys_info()?;
    Ok(())
}
