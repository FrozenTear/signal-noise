# Two Years Out-of-Tree: The famfs Merge Debate Continues

famfs is a filesystem for fabric-attached memory. John Groves at Micron Technology has been submitting it to the kernel mailing lists since 2024. It is still not in mainline. LSFMM+BPF 2026 brought it back for another round of debate, this time with FUSE and BPF in the room together.

The hardware problem famfs solves is real. CXL interconnects allow memory to be attached to a fabric and made visible to multiple hosts simultaneously. That memory exists outside the NUMA hierarchy the kernel has spent decades learning to manage. Accessing it safely — coordinating which host can write to which region, mapping it into address spaces, persisting the metadata that makes it a filesystem rather than a raw shared slab — requires something. famfs is that something, and has been since Groves posted the first version.

The project's own README is precise about scope: famfs is for fabric-attached memory generally, designed to support the broader class of fabric-attached memory — CXL is the leading carrier today, not the exclusive target. That distinction matters when you're reading the merge debate, because the objections aren't really about CXL — they're about whether the kernel should grow a new in-kernel filesystem to manage this memory class, or whether existing kernel infrastructure can be extended to do the same job.

Groves posted the v9 patch series on March 23, 2026. He believed the code was close. LSFMM+BPF 2026 said not yet.

The specific friction point: Joanne Koong raised the question of whether the famfs-specific logic should move into a BPF program operating over a generic FUSE iomap-dax interface, rather than landing as a standalone kernel filesystem. The argument is that you get the same functionality — shared-memory coordination, address space mapping — without committing to a new in-kernel surface. Miklos Szeredi, who maintains FUSE, is on record as preferring not to add famfs-specific interfaces to FUSE. His preference: try the BPF path first before merging the existing patch set.

"Try this other approach first" in kernel-land is not a refusal. It is also not an approval. It is a conditional, and conditionals take merge windows to fulfill.

famfs has already gone through a significant architectural pivot. It surfaced in 2024 as a native kernel filesystem, returned in 2025 with a FUSE implementation as an alternative path, and has been accumulating review cycles ever since. By v9 the code is, by most accounts, mature. The debate at LSFMM+BPF was not about code quality. It was about whether the kernel should have this feature at all in its current form, or whether a BPF-over-FUSE approach could absorb the use case without adding a new filesystem to the maintenance register.

The underlying hardware use case is not what is being debated. Shared memory on CXL is shipping. Software that manages access to it is needed. The question is what layer of the kernel stack carries that responsibility. famfs is one answer. The FUSE/BPF path is a potential alternative that does not yet exist.

Until one of those is conclusively in or out, famfs v9 will wait for a merge window that keeps not arriving.
