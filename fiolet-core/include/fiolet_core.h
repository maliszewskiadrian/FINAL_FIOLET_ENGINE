#ifndef FIOLET_CORE_H
#define FIOLET_CORE_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>
#include <stdbool.h>

/*
 * Binary safety decision.
 * MUST remain stable (ABI contract).
 */
typedef enum {
    FIOLET_CONTINUE = 0,
    FIOLET_ATOMIC_HALT = 1
} FioletDecision;

/*
 * Opaque kernel handle.
 * Internal layout is hidden by design.
 */
typedef struct FioletKernel FioletKernel;

/*
 * Create a new safety kernel instance.
 *
 * deviation_limit:
 *   Threshold above which the kernel irreversibly halts.
 */
FioletKernel* fiolet_kernel_new(float deviation_limit);

/*
 * Evaluate a deviation signal.
 *
 * Returns:
 *   FIOLET_CONTINUE    - safe to continue
 *   FIOLET_ATOMIC_HALT - irreversible halt
 *
 * NOTE:
 *   Once ATOMIC_HALT is returned, all future calls
 *   MUST also return ATOMIC_HALT.
 */
FioletDecision fiolet_kernel_evaluate(
    FioletKernel* kernel,
    float deviation
);

/*
 * Query halted state.
 *
 * Returns true iff the kernel has latched ATOMIC_HALT.
 */
bool fiolet_kernel_is_halted(const FioletKernel* kernel);

/*
 * Destroy kernel instance.
 *
 * NOTE:
 *   Optional in many safety-critical embeddings.
 *   Provided for host-side hygiene only.
 */
void fiolet_kernel_free(FioletKernel* kernel);

#ifdef __cplusplus
}
#endif

#endif /* FIOLET_CORE_H */
