use mediatype::MediaType;

fn main() {
    let mime_types: Vec<MediaType> = vec![];

    mime_types.iter().filter(is_allowed);
}

fn is_allowed(_media_type: &MediaType) -> bool {
    true
}
