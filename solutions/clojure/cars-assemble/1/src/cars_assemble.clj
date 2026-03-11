(ns cars-assemble
  (:require
   [clojure.math :as math]))

(def base-production 221)

(def success-rates
  (into [] cat
        [[0] (repeat 4 1) (repeat 4 0.9) [0.8 0.77]]))

(defn production-rate
  "Returns the assembly line's production rate per hour,
   taking into account its success rate"
  [speed]
  (* base-production speed (get success-rates speed)))

(defn working-items
  "Calculates how many working cars are produced per minute"
  [speed]
  (math/floor-div (production-rate speed) 60))
