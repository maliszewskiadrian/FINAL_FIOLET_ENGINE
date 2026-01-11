#include "fiolet_core.h"
#include <stdio.h>

int main(void) {
    FioletKernel* k = fiolet_kernel_new(1.0f);

    printf("halted: %d\n", fiolet_kernel_is_halted(k));

    FioletDecision d1 = fiolet_kernel_evaluate(k, 0.5f);
    printf("decision: %d\n", d1);

    FioletDecision d2 = fiolet_kernel_evaluate(k, 2.0f);
    printf("decision: %d\n", d2);

    FioletDecision d3 = fiolet_kernel_evaluate(k, 0.0f);
    printf("decision: %d\n", d3);

    fiolet_kernel_free(k);
    return 0;
}
