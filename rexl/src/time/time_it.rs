use std::time::Instant;

type FnBox = Box<dyn Fn() + 'static>;

pub struct Timeit {
    /// multi metering
    count:    u32,
    /// assert skip < count
    /// such as count=100, skip=10, then discard head 10 & tail 10 before merge
    skip:     u32,
    /// repeat times, one action avg time is `the result or run()` / repeat
    /// such as one action cost 1ms, if repeat=12 then total cost almost equal 12ms
    repeat:   u32,
    /// use parallel io to execute
    parallel: bool,
    ///v
    actions:  Vec<FnBox>,
}

impl Timeit {
    pub fn new() -> Self {
        Timeit {
            count:    1,
            skip:     0,
            repeat:   1,
            parallel: false,
            actions:  vec![],
        }
    }

    pub fn count(&mut self, count: u32) -> &mut Self {
        self.count = count;
        self
    }

    pub fn skip(&mut self, skip: u32) -> &mut Self {
        self.skip = skip;
        self
    }

    pub fn repeat(&mut self, repeat: u32) -> &mut Self {
        self.repeat = repeat;
        self
    }

    pub fn parallel(&mut self, parallel: bool) -> &mut Self {
        self.parallel = parallel;
        self
    }

    pub fn add_action<F: Fn() + 'static>(&mut self, action: F) -> &mut Self {
        self.actions.push(Box::new(action));
        self
    }

    pub fn add_unary_action<T: 'static>(
        &mut self, supplier: fn() -> T, action: fn(T),
    ) -> &mut Self {
        self.actions.push(Box::new(move || action(supplier())));
        self
    }

    pub fn run(&self) -> Vec<u64> {
        let mut tss: Vec<Vec<u64>>;

        // parallel
        if self.parallel {
            tss = self.run_parallel();
        }
        // serially
        else {
            tss = self.run_serially();
        }

        // stat
        self.stat(&mut tss)
    }

    pub fn run_serially(&self) -> Vec<Vec<u64>> {
        let size = self.actions.len();
        let count = self.count as usize;
        let repeat = self.repeat;

        let mut tss = Vec::with_capacity(size);
        for i in 0..size {
            let mut ts = Vec::with_capacity(count);
            let action = &self.actions[i];

            for _ in 0..count {
                let mut t = 0u64;
                for _ in 0..repeat {
                    let now = Instant::now();
                    action();
                    t += now.elapsed().as_nanos() as u64;
                }
                ts.push(t);
            }
            tss.push(ts);
        }
        tss
    }

    pub fn run_parallel(&self) -> Vec<Vec<u64>> {
        unimplemented!()
    }

    fn stat(&self, tss: &mut Vec<Vec<u64>>) -> Vec<u64> {
        let size = tss.len();
        let count = self.count as usize;
        let skip = self.skip as usize;

        let mut stat = vec![0u64; size];
        for i in 0..size {
            tss[i].sort();
            let mut avg = 0u64;
            let mut c = 0;
            for j in 0..count {
                // if skip = 10, then
                // skip 0, 1, ..., 9 or len-10, ..., len - 1
                if j < skip || j >= count - skip {
                    continue
                }
                c += 1;
                avg += tss[i][j];
            }
            stat[i] = avg / c;
        }
        stat
    }

    pub fn run_and_format(
        &self, unit: &'static str, unit_base: f64, delimiter: &'static str,
    ) -> String {
        let ts: Vec<String> = self
            .run()
            .iter()
            .map(|it| format!("{:?}{}", *it as f64 / unit_base, unit))
            .collect();
        ts.join(delimiter)
    }

    pub fn run_and_format_sec(&self, delimiter: &'static str) -> String {
        self.run_and_format("s", 1000_000_000.0, delimiter)
    }

    pub fn run_and_format_ms(&self, delimiter: &'static str) -> String {
        self.run_and_format("ms", 1000_000.0, delimiter)
    }

    pub fn run_and_format_us(&self, delimiter: &'static str) -> String {
        self.run_and_format("us", 1000.0, delimiter)
    }
}
