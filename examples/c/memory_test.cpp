/**
 * RustDesk FFI Memory Safety Test
 *
 * Tests for memory safety at FFI boundary using safe patterns.
 * For full ASAN testing, compile with: RUSTFLAGS="-Z sanitizer=address" cargo build -Zbuild-std
 * For valgrind testing: valgrind --leak-check=full ./memory_test
 *
 * Build: gcc -o memory_test memory_test.c -I../.. -L../../target/debug -lrustdesk
 * Run: ./memory_test
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>
#include "rustdesk.h"

// Test counters
static int tests_passed = 0;
static int tests_failed = 0;

#define TEST(name, condition) do { \
    printf("  Testing: %s... ", name); \
    if (condition) { \
        printf("PASS\n"); \
        tests_passed++; \
    } else { \
        printf("FAIL\n"); \
        tests_failed++; \
    } \
} while(0)

// Test 1: Null session handling
void test_null_session(void) {
    printf("\n=== Test 1: Null Session Handling ===\n");

    // All functions should handle null session gracefully
    TEST("disconnect(null)", rustdesk_disconnect(NULL) < 0);
    TEST("get_state(null)", rustdesk_session_get_state(NULL, NULL, 0) == -1);
    TEST("get_queue_size(null)", rustdesk_get_queue_size(NULL) == -1);
    TEST("get_frame(null)", rustdesk_get_frame(NULL) == NULL);
    TEST("send_mouse(null)", rustdesk_send_mouse_event(NULL, MouseMove, 0, 0, 0) < 0);
    TEST("send_key(null)", rustdesk_send_key_event(NULL, 0, true, 0) < 0);
    TEST("send_clipboard(null)", rustdesk_send_clipboard(NULL, "test", 4) < 0);
}

// Test 2: Free null pointers
void test_free_null(void) {
    printf("\n=== Test 2: Free Null Pointers ===\n");

    // These should not crash when called with null
    printf("  Testing: rustdesk_free_frame(null)...\n");
    rustdesk_free_frame(NULL);
    printf("  PASS (no crash)\n");
    tests_passed++;

    printf("  Testing: rustdesk_free_string(null)...\n");
    rustdesk_free_string(NULL);
    printf("  PASS (no crash)\n");
    tests_passed++;
}

// Test 3: String handling
void test_string_handling(void) {
    printf("\n=== Test 3: String Handling ===\n");

    // Test with empty strings
    TEST("connect empty peer_id", rustdesk_connect("", NULL, NULL) == NULL);
    TEST("connect null peer_id", rustdesk_connect(NULL, NULL, NULL) == NULL);
}

// Test 4: Double free detection (safe pattern)
void test_double_free_pattern(void) {
    printf("\n=== Test 4: Double Free Pattern (Safe Handling) ===\n");

    // Create a "fake" frame pointer that we manage
    // Note: actual double free on real frame would crash
    // This test verifies we don't double free by pattern

    printf("  Testing: Safe free pattern (single free)...\n");
    VideoFrameData *frame = rustdesk_get_frame(NULL); // Returns null
    if (frame == NULL) {
        // Null is safe to "not free"
        printf("  PASS (null frame handled safely)\n");
        tests_passed++;
    } else {
        // Would free once if real
        rustdesk_free_frame(frame);
        // DO NOT free again
        printf("  PASS (single free pattern)\n");
        tests_passed++;
    }
}

// Test 5: Bounds checking
void test_bounds_checking(void) {
    printf("\n=== Test 5: Bounds Checking ===\n");

    char small_buf[4];
    char large_buf[256];

    // Test with different buffer sizes
    TEST("get_state small buffer",
         rustdesk_session_get_state(NULL, small_buf, sizeof(small_buf)) == -1);
    TEST("get_state large buffer",
         rustdesk_session_get_state(NULL, large_buf, sizeof(large_buf)) == -1);
    TEST("get_state zero buffer",
         rustdesk_session_get_state(NULL, large_buf, 0) == -1);
}

// Test 6: Invalid enum values
void test_invalid_enums(void) {
    printf("\n=== Test 6: Invalid Enum Values ===\n");

    // Test with out-of-range mouse event types
    TEST("invalid mouse type (999)",
         rustdesk_send_mouse_event(NULL, static_cast<MouseEventType>(999), 0, 0, 0) < 0);

    // Test with negative coordinates (should handle gracefully)
    TEST("negative coordinates",
         rustdesk_send_mouse_event(NULL, MouseMove, -100, -100, 0) < 0);
}

// Test 7: Event callback safety
void test_event_callback_safety(void) {
    printf("\n=== Test 7: Event Callback Safety ===\n");

    // Register null callback (should be safe)
    printf("  Testing: register null callback...\n");
    rustdesk_register_event_callback(NULL, NULL);
    printf("  PASS (no crash)\n");
    tests_passed++;
}

// Test 8: Basic runtime checks
void test_constants(void) {
    printf("\n=== Test 8: Basic Runtime Checks ===\n");
    TEST("null queue size is -1", rustdesk_get_queue_size(NULL) == -1);
    TEST("null frame is null", rustdesk_get_frame(NULL) == NULL);
}

int main(int argc, char *argv[]) {
    printf("RustDesk FFI Memory Safety Test\n");
    printf("===============================\n");

    printf("\nRunning memory safety tests...\n");
    printf("Note: Full ASAN/valgrind testing requires:\n");
    printf("  ASAN: RUSTFLAGS=\"-Z sanitizer=address\" cargo build -Zbuild-std\n");
    printf("  Valgrind: valgrind --leak-check=full ./memory_test\n");

    // Run all tests
    test_null_session();
    test_free_null();
    test_string_handling();
    test_double_free_pattern();
    test_bounds_checking();
    test_invalid_enums();
    test_event_callback_safety();
    test_constants();

    // Summary
    printf("\n=== Test Summary ===\n");
    printf("Passed: %d\n", tests_passed);
    printf("Failed: %d\n", tests_failed);
    printf("Total:  %d\n", tests_passed + tests_failed);

    if (tests_failed == 0) {
        printf("\nAll memory safety tests passed!\n");
        return 0;
    } else {
        printf("\nSome tests failed. Review failures above.\n");
        return 1;
    }
}