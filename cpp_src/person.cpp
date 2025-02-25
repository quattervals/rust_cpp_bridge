#include <chrono>

#include "Person.h"

// Constructor implementation
Person::Person(const std::string& name, const std::string& zip, uint32_t dob)
    : name(name), zip(zip), dob(dob), id(0) {}

// Get age implementation
uint32_t Person::getAge() const {
    auto now = std::chrono::system_clock::now();
    std::time_t time = std::chrono::system_clock::to_time_t(now);
    std::tm* date = std::localtime(&time);


    return (tm->tm_year + 1900) - dob;
}

// Get zip implementation
const std::string& Person::getZip() const {
    return zip;
}

// Update zip implementation
void Person::updateZip(const std::string& zip) {
    this->zip = zip;
}

// Factory function implementation
std::unique_ptr<Person> newPerson(const std::string& name, const std::string& zip, uint32_t dob) {
    return std::make_unique<Person>(name, zip, dob);
}
