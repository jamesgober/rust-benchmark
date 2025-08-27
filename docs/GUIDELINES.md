# RUST LIBRARY DEVELOPMENT GUIDELINES

> **CRITICAL**: This document defines the PRIMARY and REQUIRED rules for all Rust library development. These guidelines are designed to be both AI-friendly and human-readable. All code contributions MUST adhere to these principles.

## ⚡ CORE PRINCIPLES (REQUIRED)

### 1. **HIGH PERFORMANCE** 
- Target: Sub-nanosecond overhead for timing operations
- All hot paths must be `#[inline(always)]`
- Zero allocations in timing-critical sections
- Benchmark every optimization decision

### 2. **HIGH CONCURRENCY**
- Support 100,000+ concurrent measurements
- Lock-free data structures where possible
- Thread-local storage for per-thread metrics
- Atomic operations for shared state

### 3. **MAXIMUM EFFICIENCY**
- Peak energy efficiency - lowest CPU/memory overhead
- Pre-allocated buffers (capacity hints: 32 for spans, 1024 for metrics)
- Reuse allocations through object pools
- Compile-time feature elimination

### 4. **ASYNCHRONOUS NATIVE**
- First-class async/await support
- Thread-safe by default
- Compatible with all major async runtimes (Tokio, async-std)
- Non-blocking operations only

### 5. **MAXIMUM SCALABILITY**
- Horizontal scaling through aggregation
- Future-proof API design
- Backward compatibility guaranteed
- Modular architecture for extensibility

### 6. **CROSS-PLATFORM**
- Equal support for Linux, macOS, Windows
- Platform-specific optimizations behind abstractions
- Consistent behavior across all platforms
- CI testing on all target platforms

### 7. **ROBUST**
- Graceful degradation under load
- Never panic in production code
- Comprehensive error handling
- Recovery from all failure modes

### 8. **SECURITY**
- No timing attacks through benchmark data
- Sanitized output options
- Authentication for metrics endpoints
- Constant-time operations where security matters

### 9. **BEST PRACTICES**
- **SOLID**: Single responsibility for each component
- **DRY**: Centralized timing logic, no duplication
- **KISS**: Simple API, complex implementation
- **YAGNI**: Start minimal, expand based on real needs

## 📏 DEVELOPMENT STANDARDS

### Code Quality Requirements

```rust
// GOOD: Zero-cost abstraction
#[inline(always)]
pub fn measure<T, F: FnOnce() -> T>(name: &'static str, f: F) -> T {
    #[cfg(feature = "benchmarking")]
    let _guard = bench_guard(name);
    f()
}

// BAD: Runtime overhead even when disabled
pub fn measure<T>(name: &str, f: impl FnOnce() -> T) -> T {
    let start = Instant::now();
    let result = f();
    log_timing(name, start.elapsed());
    result
}
```

### Testing Requirements
- **Unit Tests**: 100% coverage for core functionality
- **Benchmark Tests**: Prove zero overhead when disabled
- **Integration Tests**: Real-world usage patterns
- **Property Tests**: For statistical functions
- **Regression Tests**: Performance regression detection

### Documentation Standards
```rust
/// Measures the execution time of a code block.
/// 
/// # Zero Overhead
/// When the `benchmarking` feature is disabled, this macro compiles to
/// just the code block with no instrumentation.
/// 
/// # Examples
/// ```
/// let result = bench!("database_query", {
///     db.query("SELECT * FROM users").await?
/// });
/// ```
/// 
/// # Performance
/// - Overhead: ~10ns when enabled
/// - Memory: 0 allocations
/// - Thread-safe: Yes
pub macro bench { ... }
```

## 🏗️ ARCHITECTURE GUIDELINES

### Feature Organization
```toml
# Core functionality only
default = ["benchmarking"]

# Each feature must:
# 1. Have a clear, single purpose
# 2. Be optional (except benchmarking)
# 3. Document its overhead
# 4. Include tests for with/without scenarios
```

### Module Structure
```
src/
├── lib.rs           # Public API only
├── core/            # Always compiled
│   ├── time.rs      # Zero-cost Duration type
│   └── macros.rs    # Core macro definitions
├── runtime/         # Feature-gated modules
│   ├── async.rs     # [feature = "async-runtime"]
│   └── stats.rs     # [feature = "statistics"]
└── export/          # Output format modules
    └── json.rs      # [feature = "export-json"]
```

### Performance Guidelines

#### Memory Management
- **Stack First**: Prefer stack allocation
- **Pre-allocate**: Use `with_capacity()` everywhere
- **Pool Resources**: Reuse expensive objects
- **Avoid Cloning**: Use references and borrowing

#### Concurrency Patterns
```rust
// GOOD: Lock-free with atomics
use std::sync::atomic::{AtomicU64, Ordering};
static COUNTER: AtomicU64 = AtomicU64::new(0);

// GOOD: Thread-local for per-thread data
thread_local! {
    static BUFFER: RefCell<Vec<u8>> = RefCell::new(Vec::with_capacity(1024));
}

// BAD: Global mutex
static DATA: Mutex<HashMap<String, u64>> = Mutex::new(HashMap::new());
```

## 🚀 IMPLEMENTATION CHECKLIST

Before ANY code is written:
- [ ] Will this work with zero overhead when disabled?
- [ ] Is this the simplest solution that works?
- [ unexpected char ��️ and expect behavior edge cases
- [ ] Does this maintain backward compatibility?
- [ ] Are errors handled gracefully?

During implementation:
- [ ] Profile before optimizing
- [ ] Benchmark the implementation
- [ ] Document performance characteristics
- [ ] Add comprehensive tests
- [ ] Update examples

Before merging:
- [ ] All CI checks pass on all platforms
- [ ] Benchmarks show no regression
- [ ] Documentation is complete
- [ ] Breaking changes are justified
- [ ] Security implications considered

## 🔍 SPECIFIC REQUIREMENTS

### API Design
- Intuitive naming that follows Rust conventions
- Builder pattern for complex configuration
- Sane defaults that work out of the box
- Progressive disclosure of complexity

### Error Handling
- Custom error types implementing std::error::Error
- Descriptive error messages with context
- Actionable error recovery suggestions
- Chain errors to preserve root cause

### Dependency Management
- Minimal dependencies (audit every addition)
- Optional features for heavy dependencies
- Regular dependency updates and audits
- Pin only when absolutely necessary

## 🚫 ANTI-PATTERNS TO AVOID

1. **String Allocations**: Use `&'static str` for names
2. **Dynamic Dispatch**: Prefer generics over trait objects
3. **Blocking Operations**: Everything must be non-blocking
4. **Global State**: Use thread-local or passed context
5. **Unsafe Code**: Only with extensive justification
6. **Complex APIs**: If it needs extensive docs, simplify it
7. **Feature Creep**: Start minimal, expand based on usage

## 📊 SUCCESS METRICS

A library succeeds when:
- It solves a real problem elegantly
- API is intuitive without reading docs
- Performance meets or exceeds alternatives
- Works identically across all platforms
- Zero surprises in production
- Community chooses it over alternatives

## 🎓 EXAMPLE: PERFECT LIBRARY API

```rust
// Intuitive builder pattern
let client = MyLibrary::builder()
    .with_timeout(Duration::from_secs(30))  // Optional configuration
    .build()?;  // Sane defaults for everything else

// Simple common case
let result = client.do_something("input")?;

// Progressive complexity when needed
let advanced = client
    .do_something_advanced()
    .with_option(true)
    .custom_handler(|x| x * 2)
    .execute()
    .await?;
```

---

## 📚 LIBRARY SPECIFIC GUIDELINES

### BENCHMARK LIBRARY SPECIFIC

#### Project Mission
Create a hybrid benchmarking library that serves both development benchmarking (like Criterion) and production runtime monitoring, with true zero-overhead when disabled and nanosecond precision when enabled.

#### Timing Precision
- Use `std::time::Instant` for monotonic timing
- Store raw nanoseconds as `u128` internally
- Provide convenient conversion methods
- Never lose precision in calculations

#### Web Application Integration
- Support request lifetime tracking (TTFB)
- Middleware-friendly API design
- Correlation ID support
- Distributed tracing compatible

#### Data Export
- Streaming-first design (don't buffer everything)
- Configurable precision for exports
- Standard format compliance (OpenTelemetry, Prometheus)
- Compression for historical data

#### Benchmark-Specific Success Metrics
- Zero overhead proven by benchmarks (< 1ns when disabled)
- Scales to 1M+ measurements/second
- Production users report no performance impact
- Can replace both Criterion and custom timing code

#### Example Implementation
```rust
use benchmark::{bench, Benchmark};

// Development: Automatic benchmarking
#[bench]
fn process_data(input: &[u8]) -> Result<Vec<u8>> {
    // Your actual implementation
}

// Production: Zero-overhead monitoring
async fn handle_request(req: Request) -> Response {
    bench!("total_request", {
        let data = bench!("parsing", { parse(req)? });
        let result = bench!(async "processing", { process(data).await? });
        bench!("serialization", { serialize(result) })
    })
}

// Results available in any format
let metrics = Benchmark::current();
println!("p95 latency: {}", metrics.p95().as_millis());
```

---

**Remember**: Every line of code should make the library faster, simpler, or more reliable. If it doesn't do any of these, it doesn't belong.

*This document is the source of truth for all development decisions. When in doubt, optimize for simplicity and performance.*