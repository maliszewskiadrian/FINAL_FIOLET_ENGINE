from fiolet_kernel import FioletKernel

kernel = FioletKernel(deviation_limit=0.5)

signals = [0.1, 0.2, 0.4, 0.6, 0.1]

for step, dev in enumerate(signals):
    decision = kernel.evaluate(dev)

    print(
        f"step={step} deviation={dev:.2f} "
        f"decision={'HALT' if decision else 'CONTINUE'}"
    )

    if decision == FioletKernel.ATOMIC_HALT:
        print(">>> ATOMIC HALT TRIGGERED <<<")
        break
