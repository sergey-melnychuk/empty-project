#include "gtest/gtest.h"
#include "main/hello.h"

TEST(HelloTests, Greeting) {
    EXPECT_EQ(Hello("World"), "Hello, World!");
}

