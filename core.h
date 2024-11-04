#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

enum class TodoStatus {
  NotStarted,
  InProgress,
  Completed,
};

struct Todo {
  int32_t id;
  const char *title;
  TodoStatus status;
};

extern "C" {

/// # Safety
Todo *new_todo(int32_t id, const char *title);

/// # Safety
void todo_free(Todo *o);

/// # Safety
void free_string(char *s);

char *status_str(const Todo *self);

bool is_completed(const Todo *self);

const char *title(const Todo *self);

void change_status(Todo *self, TodoStatus status);

/// # Safety
void change_title(Todo *self, const char *title);

}  // extern "C"
