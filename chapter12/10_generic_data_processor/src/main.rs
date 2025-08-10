use std::marker::PhantomData;

trait DataSource {
    type Item;
    
    fn next(&mut self) -> Option<Self::Item>;
}
trait DataSink<T> {
    fn send(&mut self, item: T);
}
trait Transformer<In, Out> {
    fn transform(&self, input: In) -> Out;
}
struct DataProcessor<Source, Transform, Sink, Out>
where
    Source: DataSource,
    Transform: Transformer<Source::Item, Out>,
    Sink: DataSink<Out>,
{
    source: Source,
    transformer: Transform,
    sink: Sink,
    _phantom: PhantomData<Out>,
}
impl<Source, Transform, Sink, Out> DataProcessor<Source, Transform, Sink, Out>
where
    Source: DataSource,
    Transform: Transformer<Source::Item, Out>,
    Sink: DataSink<Out>,
{
    fn new(source: Source, transformer: Transform, sink: Sink) -> Self {
        Self {
            source,
            transformer,
            sink,
            _phantom: PhantomData,
        }
    }
    
    fn process_all(&mut self) {
        while let Some(item) = self.source.next() {
            let transformed = self.transformer.transform(item);
            self.sink.send(transformed);
        }
    }
}
// Example implementations
struct NumberGenerator {
    current: i32,
    max: i32,
}
impl DataSource for NumberGenerator {
    type Item = i32;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.max {
            self.current += 1;
            Some(self.current)
        } else {
            None
        }
    }
}
struct Doubler;
impl Transformer<i32, i32> for Doubler {
    fn transform(&self, input: i32) -> i32 {
        input * 2
    }
}

struct ToString;
impl Transformer<i32, String> for ToString {
    fn transform(&self, input: i32) -> String {
        format!("Number: {}", input)
    }
}

struct Printer;
impl DataSink<i32> for Printer {
    fn send(&mut self, item: i32) {
        println!("Processed: {}", item);
    }
}

struct StringPrinter;
impl DataSink<String> for StringPrinter {
    fn send(&mut self, item: String) {
        println!("Processed: {}", item);
    }
}
fn main() {
    println!("=== i32 -> i32 transformation ===");
    let source1 = NumberGenerator { current: 0, max: 3 };
    let transformer1 = Doubler;
    let sink1 = Printer;
    
    let mut processor1 = DataProcessor::new(source1, transformer1, sink1);
    processor1.process_all();
    
    println!("\n=== i32 -> String transformation ===");
    let source2 = NumberGenerator { current: 0, max: 3 };
    let transformer2 = ToString;
    let sink2 = StringPrinter;
    
    let mut processor2 = DataProcessor::new(source2, transformer2, sink2);
    processor2.process_all();
}
