#include "core.h"
#include <cstdlib>
#include <iostream>

int main() {
  auto todo = new_todo(1, "Learn Rust");

  auto status = status_str(todo);

  std::cout << "Todo ID: " << todo->id << std::endl;
  std::cout << "Todo Status: " << status << std::endl;
  std::cout << "Todo Title: " << todo->title << std::endl;

	change_status(todo, TodoStatus::Completed);

  std::cout << "Todo Completed: " << (is_completed(todo) ? "Yes" : "No") << std::endl;

  free_string(status);
  todo_free(todo);

  return 0;
}
