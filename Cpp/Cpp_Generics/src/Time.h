/*-------------------------------------------------------------------
  Time.h defines Time class
  - Uses chrono to implement class for updateable time instances
*/
#include <iostream>
#include <string>
#include <chrono>
#include <ctime>

namespace Points {

  class Time {
    public:
      Time();
      time_t getTime();
      tm getLocalTime();
      tm getGMTTime();
      std::string getTimeZone();
      std::string toString();
      size_t year();
      size_t month();
      size_t day();
      size_t hour();
      size_t minutes();
      size_t seconds();
    private:
      std::chrono::time_point<std::chrono::system_clock> tp;
      std::tm calTime;
      std::string dateTimeSuffix;
  };
  /*-----------------------------------------------
    Construct instance holding time_point for
    std::chrono::system_clock's epoch
    - epoch is number of seconds since 1 January 1970 UTC
    - epoch may vary with clock, e.g.,
      system_clock, high_resolution_clock
    - time_point is a structure holding chrono::duration
      for the clock's epoch
  */
  Time::Time() {
    tp = std::chrono::system_clock::now();
    calTime = getLocalTime();
  }
  /*-----------------------------------------------
    time_t is an integral type holding number of
    seconds in the current time_point
  */
  std::time_t Time::getTime() {
    return std::chrono::system_clock::to_time_t(tp); 
  }
  /*-----------------------------------------------
    returns datetime string
    - Wed Feb 21 10:18:12 2024 local_time_zone
  */
  std::string Time::toString() {
    struct tm time;
    time_t tt = getTime();
    /* compute for GMT zone */
    if(dateTimeSuffix == "GMT") {
      gmtime_s(&time, &tt);
    } 
    /* compute for local time zone*/
    else {
      localtime_s(&time, &tt);
    }
    std::string rs = asctime(&time);
    rs.resize(rs.size() - 1);  // remove trailing newline
    rs += " " + dateTimeSuffix;
    return rs;
  }
  /*-----------------------------------------------
    tm is structure holding components of calendar
    date and time, e.g., tm_sec, tm_min, ...
  */
  tm Time::getLocalTime() {
    time_t tt = getTime();
    localtime_s(&calTime, &tt);
    dateTimeSuffix = "local time zone";
    return calTime;
  }
  /*-----------------------------------------------
    tm is structure holding components of calendar
    date and time, e.g., tm_sec, tm_min, ...
  */
  tm Time::getGMTTime() {
    time_t tt = getTime();
    gmtime_s(&calTime, &tt);
    dateTimeSuffix = "GMT";
    return calTime;
  }
  /*---------------------------------------------
    methods to retrieve dateTime components
  */
  std::string Time::getTimeZone() {
    return dateTimeSuffix;
  }
  size_t Time::year() {
    auto yr = calTime.tm_year + 1900;
    return yr;
  }
  size_t Time::month() {
    auto mn = calTime.tm_mon + 1;
    return mn;
  }
  size_t Time::day() {
    auto d = calTime.tm_mday;
    return d;
  }
  size_t Time::hour() {
    auto hr = calTime.tm_hour;
    return hr;
  }
  size_t Time::minutes() {
    auto min = calTime.tm_min;
    return min;
  }
  size_t Time::seconds() {
    double sec = calTime.tm_sec;
    return sec;
  }
}