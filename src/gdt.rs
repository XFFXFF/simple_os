use lazy_static::lazy_static;
use x86_64::structures::gdt::SegmentSelector;
use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable};
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtAddr;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

// Create a new TSS that contains a separate double fault stack in its interrupt
// stack table. The CPU will then always switch to the specified stack before the
// double fault handler is invoked. This allows kernels to recover from corrupt
// stack pointers (e.g., on kernel stack overflow).
lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            // TODO: why 5 page size?
            const STACK_SIZE: usize = 4096 * 5;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(unsafe { &STACK });
            let stack_end = stack_start + STACK_SIZE;
            // return top address because stacks on x86 grow downwards,
            // i.e. from high addresses to low addresses.
            stack_end
        };
        tss
    };
}

struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

// Since the TSS uses the segmentation system (for historical reasons), the CPU
// can not load the table directly, we need to add a new segment descriptor to
// the Global Descriptor Table (GDT)
lazy_static! {
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
        (
            gdt,
            Selectors {
                code_selector,
                tss_selector,
            },
        )
    };
}

pub fn init() {
    use x86_64::instructions::segmentation::set_cs;
    use x86_64::instructions::tables::load_tss;

    GDT.0.load();
    unsafe {
        // reload the code segment register using set_cs
        set_cs(GDT.1.code_selector);
        // load the TSS using load_tss
        load_tss(GDT.1.tss_selector);
    }
}
