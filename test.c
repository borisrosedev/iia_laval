#include <CUnit/CUnit.h>
#include <CUnit/Basic.h>

void test_function1(void);
void test_function2(void);

int main() {
    CU_initialize_registry();
    CU_pSuite suite = CU_add_suite("Main Suite", 0, 0);
    CU_add_test(suite, "Test Function 1", test_function1);
    CU_add_test(suite, "Test Function 2", test_function2);
    CU_basic_set_mode(CU_BRM_VERBOSE);
    CU_basic_run_tests();
    CU_cleanup_registry();
    return 0;
}

void test_function1(void) {
    CU_ASSERT(1 + 1 == 2);
}

void test_function2(void) {
    CU_ASSERT(2 * 2 == 4);
}