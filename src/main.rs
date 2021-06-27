use bindings::Windows;

fn location()->Result<Windows::Devices::Geolocation::BasicGeoposition, windows::Error>{          
        let geolocator = Windows::Devices::Geolocation::Geolocator::new();
        let pos = geolocator?.GetGeopositionAsync();
        Ok(pos?.get()?.Coordinate()?.Point()?.Position()?)
}

fn main() {
    println!("{:?}",location().ok());    
}
