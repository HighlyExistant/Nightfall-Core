use ash::Entry;
lazy_static::lazy_static! { 
    pub static ref ENTRY: Entry = Entry::linked();
}