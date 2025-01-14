# project name
TARGET   = port_scanner

# compiler and compiler flags
CC       = g++
CFLAGS   = -Wall -Wextra -I. -g

# linker and linker flags
LINKER   = g++
LFLAGS   = -Wall -I. -lm -g

SRCDIR   = src
INCDIR   = inc
OBJDIR   = obj
BINDIR   = bin

SOURCES  := $(wildcard $(SRCDIR)/*.cpp)   # Change .c to .cpp
INCLUDES := $(wildcard $(INCDIR)/*.h)
OBJECTS  := $(SOURCES:$(SRCDIR)/%.cpp=$(OBJDIR)/%.o)  # Change .c to .cpp
rm       = rm -f

$(BINDIR)/$(TARGET): $(OBJECTS)
	@$(LINKER) $(OBJECTS) $(LFLAGS) -o $@
	@echo "*****LINKING COMPLETE*****"

$(OBJECTS): $(OBJDIR)/%.o : $(SRCDIR)/%.cpp  # Change .c to .cpp
	@$(CC) $(CFLAGS) -c $< -o $@
	@echo "*****COMPILATION COMPLETE*****"

.PHONY: run
run:
	@./$(BINDIR)/$(TARGET)
	@echo "*****EXECUTION COMPLETE*****"

.PHONY: clean
clean:
	@$(rm) $(OBJECTS)
	@echo "*****CLEANUP COMPLETE*****"

.PHONY: remove
remove: clean
	@$(rm) $(BINDIR)/$(TARGET)
	@echo "*****EXECUTABLE REMOVED*****"

.PHONY: all
all: $(BINDIR)/$(TARGET)

