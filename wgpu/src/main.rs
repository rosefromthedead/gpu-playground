use wgpu::Backends;

fn main() {
    let instance = wgpu::Instance::new(Backends::PRIMARY);
    for adapter in instance.enumerate_adapters(Backends::PRIMARY) {
        println!("{:?}", adapter);
    }
}
