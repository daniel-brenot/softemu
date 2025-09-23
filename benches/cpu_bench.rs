use criterion::{black_box, criterion_group, criterion_main, Criterion};
use softemu::cpu::{CpuState, InstructionDecoder};
use softemu::memory::GuestMemory;

fn bench_instruction_decode(c: &mut Criterion) {
    let mut decoder = InstructionDecoder::new();
    let test_instructions = vec![
        vec![0x90], // NOP
        vec![0x48, 0x89, 0xC3], // mov %rax, %rbx
        vec![0x48, 0x01, 0xD8], // add %rbx, %rax
        vec![0x48, 0x83, 0xC4, 0x08], // add $8, %rsp
        vec![0xE9, 0x00, 0x00, 0x00, 0x00], // jmp +0
    ];

    c.bench_function("instruction_decode", |b| {
        b.iter(|| {
            for instruction_bytes in &test_instructions {
                black_box(decoder.decode_instruction(instruction_bytes));
            }
        })
    });
}

fn bench_memory_access(c: &mut Criterion) {
    let memory = GuestMemory::new(1024 * 1024 * 1024).unwrap();
    let mut cpu_state = CpuState::new(memory);

    c.bench_function("memory_read_u64", |b| {
        b.iter(|| {
            for i in 0..1000 {
                black_box(cpu_state.read_u64(i * 8).unwrap());
            }
        })
    });

    c.bench_function("memory_write_u64", |b| {
        b.iter(|| {
            for i in 0..1000 {
                black_box(cpu_state.write_u64(i * 8, i as u64).unwrap());
            }
        })
    });
}

fn bench_register_operations(c: &mut Criterion) {
    let memory = GuestMemory::new(1024 * 1024 * 1024).unwrap();
    let mut cpu_state = CpuState::new(memory);

    c.bench_function("register_get_set", |b| {
        b.iter(|| {
            for i in 0..16 {
                let value = cpu_state.registers.get_gp_register(i);
                cpu_state.registers.set_gp_register(i, value + 1);
            }
        })
    });
}

criterion_group!(benches, bench_instruction_decode, bench_memory_access, bench_register_operations);
criterion_main!(benches);
