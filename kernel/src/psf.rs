// From https://gist.github.com/hinzundcode/0480c5c8aa220cd43cc8da634119a3c0
const PSF2_MAGIC: [u8;4] = [0x72, 0xb5, 0x4a, 0x86];

#[derive(Debug)]
pub enum Error {
	OutOfBounds,
	InvalidMagic,
}

#[repr(C, packed)]
pub struct PSF2Header {
	magic: [u8;4],
	version: u32,
	header_size: u32,
	flags: u32,
	length: u32,
	char_size: u32,
	height: u32,
	width: u32,
}

pub struct PSF2Font<'a> {
	data: &'a [u8],
	header: &'a PSF2Header,
}

impl<'a> PSF2Font<'a> {
	pub fn parse(data: &'a [u8]) -> Result<Self, Error> {
		if data.len() < core::mem::size_of::<PSF2Header>() {
			return Err(Error::OutOfBounds);
		}
		
		let header = unsafe {
			let ref header = *(data.as_ptr() as *const PSF2Header);
			header
		};
		
		if header.magic != PSF2_MAGIC {
			return Err(Error::InvalidMagic);
		}
		
		let last_glyph_pos = header.header_size + header.char_size * (header.length - 1);
		if data.len() < last_glyph_pos as usize {
			return Err(Error::OutOfBounds);
		}
		
		Ok(PSF2Font {
			data,
			header,
		})
	}
	
	pub fn glyph_size(&self) -> (u32, u32) {
		(self.header.width, self.header.height)
	}
	
	pub fn glyph_count(&self) -> u32 {
		self.header.length
	}
	
	pub fn glyph(&self, index: u32) -> Option<&[u8]> {
		if index >= self.header.length {
			return None
		}
		
		let length = self.header.char_size as usize;
		let offset = self.header.header_size as usize + index as usize * length;
		Some(&self.data[offset..(offset+length)])
	}
}
