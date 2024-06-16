/*-------------------------------------------------------------------
  Time.h defines Time class to manage datetime strings
  - Uses chrono to implement class for updateable time instances

  Note: Add callable function for end of time period
*/
#ifndef TimeHeader
#define TimeHeader

#include <iostream>
#include <string>
#include <chrono>
#include <ctime>
#include <thread>
#include "AnalysisIter.h"

using namespace std::chrono;
using namespace Analysis;

namespace Points {

  /*---------------------------------------------
    Time manages calendar times
  */
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
    - member calTime is localtime after calling
      this function
  */
  tm Time::getLocalTime() {
    time_t tt = getTime();
    localtime_s(&calTime, &tt);  // save in calTime
    dateTimeSuffix = "local time zone";
    return calTime;
  }
  /*-----------------------------------------------
    tm is structure holding components of calendar
    date and time, e.g., tm_sec, tm_min, ...
    - member calTime is gmttime after calling
      this function
  */
  tm Time::getGMTTime() {
    time_t tt = getTime();
    gmtime_s(&calTime, &tt);  // save in calTime
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

  /*---------------------------------------------
    Timer provides elapsed time services
  */
  class Timer {
    public:
    Timer();
    void start();
    void stop();
    size_t elapsedNanoSec();
    size_t elapsedMicroSec();
    size_t elapsedMilliSec();
    private:
      std::chrono::time_point<
        std::chrono::high_resolution_clock
      > tp;
      std::chrono::time_point<std::chrono::high_resolution_clock> starttime;
      std::chrono::time_point<std::chrono::high_resolution_clock> stoptime;
  };
  Timer::Timer() {
    starttime = std::chrono::high_resolution_clock::now();
    stoptime = std::chrono::high_resolution_clock::now();
  }
  void Timer::start() {
    starttime = std::chrono::high_resolution_clock::now();
  }
  void Timer::stop() {
    stoptime = std::chrono::high_resolution_clock::now();
  }
  size_t Timer::elapsedNanoSec() {
    auto duration = duration_cast<std::chrono::nanoseconds>(stoptime - starttime);
    return duration.count();
  }
  size_t Timer::elapsedMicroSec() {
    auto duration = duration_cast<std::chrono::microseconds>(stoptime - starttime);
    return duration.count();
  }
  size_t Timer::elapsedMilliSec() {
    auto duration = duration_cast<std::chrono::milliseconds>(stoptime - starttime);
    return duration.count();
  }
}
/*-- test calendar time Time class --*/

void testtime() {
  using namespace Points;

  println();
  showNote("test Time", 20);
  Time t;
  t.getLocalTime();
  std::cout << "\n  datetime = " << t.toString() << std::endl;
  std::cout << "\n  epoch in secs = " << t.getTime();
  std::cout << "\n  year:     " << t.year();
  std::cout << "\n  month:    " << t.month();
  std::cout << "\n  day:      " << t.day();
  std::cout << "\n  hour:     " << t.hour();
  std::cout << "\n  minutes:  " << t.minutes();
  std::cout << "\n  seconds:  " << t.seconds();
  std::cout << "\n  timezone: " << t.getTimeZone();
  std::cout << std::endl;

  t.getGMTTime();
  std::cout << "\n  datetime = " << t.toString() << std::endl;
  std::cout << "\n  epoch in secs = " << t.getTime();
  std::cout << "\n  year:     " << t.year();
  std::cout << "\n  month:    " << t.month();
  std::cout << "\n  day:      " << t.day();
  std::cout << "\n  hour:     " << t.hour();
  std::cout << "\n  minutes:  " << t.minutes();
  std::cout << "\n  seconds:  " << t.seconds();
  std::cout << "\n  timezone: " << t.getTimeZone();
  std::cout << std::endl;
}

/*-- test timer Timer class --*/
void testtimer() {
  using namespace Analysis;
  using namespace Points;

  println();
  showNote("test Timer", 20);

  std::vector<double> v {
    1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0, 4.5, 5.0, 5.5
  };
  auto f = [&v]() {
    for(auto &item : v) { item *= item; }
  };

  auto g = [f](size_t n) {
    for(size_t i = 0; i < n; ++i) { f(); }
  };

  Timer tmr;
  tmr.start();
  tmr.stop();
  std::cout << "  noOp elapsed interval in nanosec = " 
            << tmr.elapsedNanoSec() << "\n";

  tmr.start();
  g(200);
  tmr.stop();
  std::cout << "  g(200) elapsed interval in nanosec = " 
            << tmr.elapsedNanoSec() << "\n";
  std::cout << "  g(200) elapsed interval in microsec = " 
            << tmr.elapsedMicroSec() << "\n";

  tmr.start();
  std::this_thread::sleep_for(std::chrono::milliseconds(5));
  tmr.stop();
  std::cout << "  5 millisec sleep elapsed interval in millisec = " 
            << tmr.elapsedMilliSec() << "\n";
}
#endif