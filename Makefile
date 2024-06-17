GIT_TREE_HASH := $(shell git write-tree)
RESULT_DIR = result
TARGET = target
SCRIPT_DIR = scripts
CARGO = cargo
MAKEFILE_PATH := $(abspath $(lastword $(MAKEFILE_LIST)))
MAKEFILE_DIR := $(dir $(MAKEFILE_PATH))
RELEASE = $(MAKEFILE_DIR)$(TARGET)/release
VENV = ./venv
PYTHON = $(VENV)/bin/python3
PIP = $(VENV)/bin/pip3
SBATCH := sbatch
NUM_EXPANSION = 100 500 1000 5000 10000 50000 100000 500000 1000000 5000000
NBinD = 1 4 8
NTRIAL = 0 $(NUM_EXPANSION)
RTDP_FLAGS = "-n 1000"
RTDPD_FLAGS = "-n 1000 --use-domain-heuristic"
GRID_VI_FLAGS = "-n 1000"
MCTS_FLAGS = "-n 1 -c 1.0 --am-split"
MCTS_FULL_FLAGS = "-n 1 -c 1.0 --am-split --full-rollouts"
USER = smiura_umass_edu

NODE_LIST = --nodelist=cpu009,cpu010,cpu011,cpu012,cpu013,cpu014,cpu015,cpu016,cpu017,cpu018,cpu019,cpu020,cpu020
TIMELIMIT = 10

$(RESULT_DIR)/$(GIT_TREE_HASH):
	mkdir -p $@

$(VENV): requirements.txt
	python3 -m venv venv
	$(PIP) install --upgrade pip
	$(PIP) install -r requirements.txt

ALGORITHMS = mcts_full lrtdp rtdp_d lrtdp_d grid_vi
HMIN_DOMAINS = baker reset reset5 baker_com baker5 baker5_com blocks blocks4_3 blocks6 spelling recycle
DOMAINS = $(HMIN_DOMAINS) simple_av
# REPETITION = 0 1 2 3 4 5 6 7 8 9
REPETITION = 0 1 2 3 4
baker_horizons = 30
reset_horizons = 30
reset5_horizons = 50
baker_com_horizons = 30
baker5_horizons = 30 
baker5_com_horizons = 30 
spelling_horizons = 50 
recycle_horizons = 20 
blocks_horizons = 50
simple_av_horizons = 13

reset_ids = 101 102 601 603 605 607 609
# reset5_ids = 301 302 303 304 401 402 403 404
reset5_ids = 302
baker_ids = 101 102 601 603 605 607 609
# baker5_com_ids = 200 201 202 300 301 302
baker5_com_ids = 201 202 203 204 301 302 303 304
# baker5_ids = 400 401 402 403 404
baker5_ids = 302 403
# spelling_ids = 3 8
spelling_ids = 8
# blocks_ids = 1 2 3
blocks_ids = 1
simple_av_ids = 1 2
recycle_ids = 10 11 12 13 14

html/$(GIT_TREE_HASH):
	mkdir -p html/$(GIT_TREE_HASH)

COMMA := ,
EMPTY :=
SPACE := $(EMPTY) $(EMPTY)

SLEEP = sleep 1
#SLEEP = 

define f_mcts =
$$(RESULT_DIR)/$$(GIT_TREE_HASH)/mcts_$(4)_$(1)_$(2)_$(3)_$(5).json: $$(RESULT_DIR)/$$(GIT_TREE_HASH)
ifeq (,$$(wildcard $$(RESULT_DIR)/$$(GIT_TREE_HASH)/mcts_$(4)_$(1)_$(2)_$(3)_$(5).o))
	$$(info Submitting a new job)
	echo $$@ >> $$(RESULT_DIR)/$$(GIT_TREE_HASH)/$(4)_jsons.txt
	$$(eval JOBID=$$(shell $$(SBATCH) --export=All,Instance=$(1),NExpansion=$(2),Horizon=$(3),Domain=$(4),Flags=$$(MCTS_FLAGS) $$(NODE_LIST) --time=$$(TIMELIMIT) --output=$$(RESULT_DIR)/$$(GIT_TREE_HASH)/mcts_$(4)_$(1)_$(2)_$(3)_$(5).o --error=$$(RESULT_DIR)/$$(GIT_TREE_HASH)/mcts_$(4)_$(1)_$(2)_$(3)_$(5).e --job-name=mcts_$(4) --parsable slurm/mcts.sh))
	$$(SBATCH) --export=All,Filename=$$(RESULT_DIR)/$$(GIT_TREE_HASH)/mcts_$(4)_$(1)_$(2)_$(3)_$(5).o --output=$$@ --error=$$(RESULT_DIR)/$$(GIT_TREE_HASH)/parse_mcts_$(4)_$(1)_$(2)_$(3)_$(5).e --job-name=parse_mcts_$(4) --dependency=$$(JOBID) slurm/parse.sh
	$$(SLEEP)
endif

.PRECIOUS: $$(RESULT_DIR)/$$(GIT_TREE_HASH)/mcts_$(4)_$(1)_$(2)_$(3)_$(5).o
endef
$(foreach domain,$(DOMAINS),$(foreach assignment_id,$($(domain)_ids),$(foreach horizon,$($(domain)_horizons),$(foreach n_expansion,$(NUM_EXPANSION),$(foreach rep,$(REPETITION),$(eval $(call f_mcts,$(assignment_id),$(n_expansion),$(horizon),$(domain),$(rep))))))))

define f_mcts_jsons =
mcts_$(1)_jsons = $$(foreach instance,$$($(1)_ids),$$(foreach n_expansion,$$(NUM_EXPANSION),$$(foreach horizon,$$($(1)_horizons),$$(foreach rep,$$(REPETITION),$$(RESULT_DIR)/$$(GIT_TREE_HASH)/mcts_$(1)_$$(instance)_$$(n_expansion)_$$(horizon)_$$(rep).json))))
endef

$(foreach domain,$(DOMAINS),$(eval $(call f_mcts_jsons,$(domain))))

define f_mcts_full =
$$(RESULT_DIR)/$$(GIT_TREE_HASH)/mcts_full_$(4)_$(1)_$(2)_$(3)_$(5).json: $$(RESULT_DIR)/$$(GIT_TREE_HASH)
ifeq (,$$(wildcard $$(RESULT_DIR)/$$(GIT_TREE_HASH)/mcts_full_$(4)_$(1)_$(2)_$(3)_$(5).o))
	$$(info Submitting a new job)
	echo $$@ >> $$(RESULT_DIR)/$$(GIT_TREE_HASH)/$(4)_jsons.txt
	$$(eval JOBID=$$(shell $$(SBATCH) --export=All,Instance=$(1),NExpansion=$(2),Horizon=$(3),Domain=$(4),Flags=$$(MCTS_FULL_FLAGS) $$(NODE_LIST) --time=$$(TIMELIMIT) --output=$$(RESULT_DIR)/$$(GIT_TREE_HASH)/mcts_full_$(4)_$(1)_$(2)_$(3)_$(5).o --error=$$(RESULT_DIR)/$$(GIT_TREE_HASH)/mcts_full_$(4)_$(1)_$(2)_$(3)_$(5).e --job-name=mcts_full_$(4) --parsable slurm/mcts.sh))
	$$(SBATCH) --export=All,Filename=$$(RESULT_DIR)/$$(GIT_TREE_HASH)/mcts_full_$(4)_$(1)_$(2)_$(3)_$(5).o --output=$$@ --error=$$(RESULT_DIR)/$$(GIT_TREE_HASH)/parse_mcts_full_$(4)_$(1)_$(2)_$(3)_$(5).e --job-name=parse_mcts_full_$(4) --dependency=$$(JOBID) slurm/parse.sh
	$$(SLEEP)
endif

.PRECIOUS: $$(RESULT_DIR)/$$(GIT_TREE_HASH)/mcts_full_$(4)_$(1)_$(2)_$(3)_$(5).o
endef
$(foreach domain,$(DOMAINS),$(foreach assignment_id,$($(domain)_ids),$(foreach horizon,$($(domain)_horizons),$(foreach n_expansion,$(NUM_EXPANSION),$(foreach rep,$(REPETITION),$(eval $(call f_mcts_full,$(assignment_id),$(n_expansion),$(horizon),$(domain),$(rep))))))))

define f_mcts_full_jsons =
mcts_full_$(1)_jsons = $$(foreach instance,$$($(1)_ids),$$(foreach n_expansion,$$(NUM_EXPANSION),$$(foreach horizon,$$($(1)_horizons),$$(foreach rep,$$(REPETITION),$$(RESULT_DIR)/$$(GIT_TREE_HASH)/mcts_full_$(1)_$$(instance)_$$(n_expansion)_$$(horizon)_$$(rep).json))))
endef

$(foreach domain,$(DOMAINS),$(eval $(call f_mcts_full_jsons,$(domain))))

define f_rtdp =
$$(RESULT_DIR)/$$(GIT_TREE_HASH)/rtdp_$(5)_$(1)_$(2)_$(3)_$(4)_$(6).json: $$(RESULT_DIR)/$$(GIT_TREE_HASH)
ifeq (,$$(wildcard $$(RESULT_DIR)/$$(GIT_TREE_HASH)/rtdp_$(5)_$(1)_$(2)_$(3)_$(4)_$(6).o))
	$$(info Submitting a new job)
	echo $$@ >> $$(RESULT_DIR)/$$(GIT_TREE_HASH)/$(5)_jsons.txt
	$$(eval JOBID=$$(shell $$(SBATCH) --export=All,Instance=$(1),NBin=$(2),NTrial=$(3),Horizon=$(4),Domain=$(5),Flags=$$(RTDP_FLAGS) $$(NODE_LIST) --time=$$(TIMELIMIT) --output=$$(RESULT_DIR)/$$(GIT_TREE_HASH)/rtdp_$(5)_$(1)_$(2)_$(3)_$(4)_$(6).o --error=$$(RESULT_DIR)/$$(GIT_TREE_HASH)/rtdp_$(5)_$(1)_$(2)_$(3)_$(4)_$(6).e --job-name=rtdp_$(5) --parsable slurm/rtdp.sh))
	$$(SBATCH) --export=All,Filename=$$(RESULT_DIR)/$$(GIT_TREE_HASH)/rtdp_$(5)_$(1)_$(2)_$(3)_$(4)_$(6).o --output=$$@ --error=$$(RESULT_DIR)/$$(GIT_TREE_HASH)/parse_rtdp_$(5)_$(1)_$(2)_$(3)_$(4)_$(6).e --job-name=parse_rtdp_$(5) --dependency=$$(JOBID) slurm/parse.sh
	$$(SLEEP)
endif

.PRECIOUS: $$(RESULT_DIR)/$$(GIT_TREE_HASH)/rtdp_$(5)_$(1)_$(2)_$(3)_$(4)_$(6).o
endef
$(foreach domain,$(DOMAINS),$(foreach assignment_id,$($(domain)_ids),$(foreach horizon,$($(domain)_horizons),$(foreach n_trial,$(NTRIAL),$(foreach rep,$(REPETITION),$(foreach n_bin,$(NBinD),$(eval $(call f_rtdp,$(assignment_id),$(n_bin),$(n_trial),$(horizon),$(domain),$(rep)))))))))

define f_rtdp_jsons =
rtdp_$(1)_jsons = $$(foreach instance,$$($(1)_ids),$$(foreach n_bin,$$(NBinD),$$(foreach n_trial,$$(NTRIAL),$$(foreach horizon,$$($(1)_horizons),$$(foreach rep,$$(REPETITION),$$(RESULT_DIR)/$$(GIT_TREE_HASH)/rtdp_$(1)_$$(instance)_$$(n_bin)_$$(n_trial)_$$(horizon)_$$(rep).json)))))
endef

$(foreach domain,$(DOMAINS),$(eval $(call f_rtdp_jsons,$(domain))))

define f_lrtdp =
$$(RESULT_DIR)/$$(GIT_TREE_HASH)/lrtdp_$(5)_$(1)_$(2)_$(3)_$(4)_$(6).json: $$(RESULT_DIR)/$$(GIT_TREE_HASH)
ifeq (,$$(wildcard $$(RESULT_DIR)/$$(GIT_TREE_HASH)/lrtdp_$(5)_$(1)_$(2)_$(3)_$(4)_$(6).o))
	$$(info Submitting a new job)
	echo $$@ >> $$(RESULT_DIR)/$$(GIT_TREE_HASH)/$(5)_jsons.txt
	$$(eval JOBID=$$(shell $$(SBATCH) --export=All,Instance=$(1),NBin=$(2),NTrial=$(3),Horizon=$(4),Domain=$(5),Flags=$$(RTDP_FLAGS) $$(NODE_LIST) --time=$$(TIMELIMIT) --output=$$(RESULT_DIR)/$$(GIT_TREE_HASH)/lrtdp_$(5)_$(1)_$(2)_$(3)_$(4)_$(6).o --error=$$(RESULT_DIR)/$$(GIT_TREE_HASH)/lrtdp_$(5)_$(1)_$(2)_$(3)_$(4)_$(6).e --job-name=lrtdp_$(5) --parsable slurm/lrtdp.sh))
	$$(SBATCH) --export=All,Filename=$$(RESULT_DIR)/$$(GIT_TREE_HASH)/lrtdp_$(5)_$(1)_$(2)_$(3)_$(4)_$(6).o --output=$$@ --error=$$(RESULT_DIR)/$$(GIT_TREE_HASH)/parse_lrtdp_$(5)_$(1)_$(2)_$(3)_$(4)_$(6).e --job-name=parse_lrtdp_$(5) --dependency=$$(JOBID) slurm/parse.sh
	$$(SLEEP)
endif

.PRECIOUS: $$(RESULT_DIR)/$$(GIT_TREE_HASH)/lrtdp_$(5)_$(1)_$(2)_$(3)_$(4)_$(6).o
endef
$(foreach domain,$(DOMAINS),$(foreach assignment_id,$($(domain)_ids),$(foreach horizon,$($(domain)_horizons),$(foreach n_trial,$(NTRIAL),$(foreach rep,$(REPETITION),$(foreach n_bin,$(NBinD),$(eval $(call f_lrtdp,$(assignment_id),$(n_bin),$(n_trial),$(horizon),$(domain),$(rep)))))))))

define f_lrtdp_jsons =
lrtdp_$(1)_jsons = $$(foreach instance,$$($(1)_ids),$$(foreach n_bin,$$(NBinD),$$(foreach n_trial,$$(NTRIAL),$$(foreach horizon,$$($(1)_horizons),$$(foreach rep,$$(REPETITION),$$(RESULT_DIR)/$$(GIT_TREE_HASH)/lrtdp_$(1)_$$(instance)_$$(n_bin)_$$(n_trial)_$$(horizon)_$$(rep).json)))))
endef

$(foreach domain,$(DOMAINS),$(eval $(call f_lrtdp_jsons,$(domain))))

define f_rtdp_d =
$$(RESULT_DIR)/$$(GIT_TREE_HASH)/rtdp_d_$(5)_$(1)_$(2)_$(3)_$(4)_$(6).json: $$(RESULT_DIR)/$$(GIT_TREE_HASH)
ifeq (,$$(wildcard $$(RESULT_DIR)/$$(GIT_TREE_HASH)/rtdp_d_$(5)_$(1)_$(2)_$(3)_$(4)_$(6).o))
	$$(info Submitting a new job)
	echo $$@ >> $$(RESULT_DIR)/$$(GIT_TREE_HASH)/$(5)_jsons.txt
	$$(eval JOBID=$$(shell $$(SBATCH) --export=All,Instance=$(1),NBin=$(2),NTrial=$(3),Horizon=$(4),Domain=$(5),Flags=$$(RTDP_FLAGS) $$(NODE_LIST) --time=$$(TIMELIMIT) --output=$$(RESULT_DIR)/$$(GIT_TREE_HASH)/rtdp_d_$(5)_$(1)_$(2)_$(3)_$(4)_$(6).o --error=$$(RESULT_DIR)/$$(GIT_TREE_HASH)/rtdp_d_$(5)_$(1)_$(2)_$(3)_$(4)_$(6).e --job-name=rtdp_d_$(5) --parsable slurm/rtdp_d.sh))
	$$(SBATCH) --export=All,Filename=$$(RESULT_DIR)/$$(GIT_TREE_HASH)/rtdp_d_$(5)_$(1)_$(2)_$(3)_$(4)_$(6).o --output=$$@ --error=$$(RESULT_DIR)/$$(GIT_TREE_HASH)/parse_rtdp_d_$(5)_$(1)_$(2)_$(3)_$(4)_$(6).e --job-name=parse_rtdp_d_$(5) --dependency=$$(JOBID) slurm/parse.sh
	$$(SLEEP)
endif

.PRECIOUS: $$(RESULT_DIR)/$$(GIT_TREE_HASH)/rtdp_d_$(5)_$(1)_$(2)_$(3)_$(4)_$(6).o
endef
$(foreach domain,$(DOMAINS),$(foreach assignment_id,$($(domain)_ids),$(foreach horizon,$($(domain)_horizons),$(foreach n_trial,$(NTRIAL),$(foreach rep,$(REPETITION),$(foreach n_bin,$(NBinD),$(eval $(call f_rtdp_d,$(assignment_id),$(n_bin),$(n_trial),$(horizon),$(domain),$(rep)))))))))

define f_rtdp_d_jsons =
rtdp_d_$(1)_jsons = $$(foreach instance,$$($(1)_ids),$$(foreach n_bin,$$(NBinD),$$(foreach n_trial,$$(NTRIAL),$$(foreach horizon,$$($(1)_horizons),$$(foreach rep,$$(REPETITION),$$(RESULT_DIR)/$$(GIT_TREE_HASH)/rtdp_d_$(1)_$$(instance)_$$(n_bin)_$$(n_trial)_$$(horizon)_$$(rep).json)))))
endef

$(foreach domain,$(DOMAINS),$(eval $(call f_rtdp_d_jsons,$(domain))))

define f_lrtdp_d =
$$(RESULT_DIR)/$$(GIT_TREE_HASH)/lrtdp_d_$(5)_$(1)_$(2)_$(3)_$(4)_$(6).json: $$(RESULT_DIR)/$$(GIT_TREE_HASH)
ifeq (,$$(wildcard $$(RESULT_DIR)/$$(GIT_TREE_HASH)/lrtdp_d_$(5)_$(1)_$(2)_$(3)_$(4)_$(6).o))
	$$(info Submitting a new job)
	echo $$@ >> $$(RESULT_DIR)/$$(GIT_TREE_HASH)/$(5)_jsons.txt
	$$(eval JOBID=$$(shell $$(SBATCH) --export=All,Instance=$(1),NBin=$(2),NTrial=$(3),Horizon=$(4),Domain=$(5),Flags=$$(RTDP_FLAGS) $$(NODE_LIST) --time=$$(TIMELIMIT) --output=$$(RESULT_DIR)/$$(GIT_TREE_HASH)/lrtdp_d_$(5)_$(1)_$(2)_$(3)_$(4)_$(6).o --error=$$(RESULT_DIR)/$$(GIT_TREE_HASH)/lrtdp_d_$(5)_$(1)_$(2)_$(3)_$(4)_$(6).e --job-name=lrtdp_d_$(5) --parsable slurm/lrtdp_d.sh))
	$$(SBATCH) --export=All,Filename=$$(RESULT_DIR)/$$(GIT_TREE_HASH)/lrtdp_d_$(5)_$(1)_$(2)_$(3)_$(4)_$(6).o --output=$$@ --error=$$(RESULT_DIR)/$$(GIT_TREE_HASH)/parse_lrtdp_d_$(5)_$(1)_$(2)_$(3)_$(4)_$(6).e --job-name=parse_lrtdp_d_$(5) --dependency=$$(JOBID) slurm/parse.sh
	$$(SLEEP)
endif

.PRECIOUS: $$(RESULT_DIR)/$$(GIT_TREE_HASH)/lrtdp_d_$(5)_$(1)_$(2)_$(3)_$(4)_$(6).o
endef
$(foreach domain,$(DOMAINS),$(foreach assignment_id,$($(domain)_ids),$(foreach horizon,$($(domain)_horizons),$(foreach n_trial,$(NTRIAL),$(foreach rep,$(REPETITION),$(foreach n_bin,$(NBinD),$(eval $(call f_lrtdp_d,$(assignment_id),$(n_bin),$(n_trial),$(horizon),$(domain),$(rep)))))))))

define f_lrtdp_d_jsons =
lrtdp_d_$(1)_jsons = $$(foreach instance,$$($(1)_ids),$$(foreach n_bin,$$(NBinD),$$(foreach n_trial,$$(NTRIAL),$$(foreach horizon,$$($(1)_horizons),$$(foreach rep,$$(REPETITION),$$(RESULT_DIR)/$$(GIT_TREE_HASH)/lrtdp_d_$(1)_$$(instance)_$$(n_bin)_$$(n_trial)_$$(horizon)_$$(rep).json)))))
endef

$(foreach domain,$(DOMAINS),$(eval $(call f_lrtdp_d_jsons,$(domain))))

define f_grid_vi =
$$(RESULT_DIR)/$$(GIT_TREE_HASH)/grid_vi_$(4)_$(1)_$(2)_$(3)_$(5).json: $$(RESULT_DIR)/$$(GIT_TREE_HASH)
ifeq (,$$(wildcard $$(RESULT_DIR)/$$(GIT_TREE_HASH)/grid_vi_$(4)_$(1)_$(2)_$(3)_$(5).o))
	$$(info Submitting a new job)
	echo $$@ >> $$(RESULT_DIR)/$$(GIT_TREE_HASH)/$(4)_jsons.txt
	$$(eval JOBID=$$(shell $$(SBATCH) --export=All,Instance=$(1),NBin=$(2),Horizon=$(3),Domain=$(4),Flags=$$(GRID_VI_FLAGS) $$(NODE_LIST) --time=$$(TIMELIMIT) --output=$$(RESULT_DIR)/$$(GIT_TREE_HASH)/grid_vi_$(4)_$(1)_$(2)_$(3)_$(5).o --error=$$(RESULT_DIR)/$$(GIT_TREE_HASH)/grid_vi_$(4)_$(1)_$(2)_$(3)_$(5).e --job-name=grid_vi_$(4) --parsable slurm/grid_vi.sh))
	$$(SBATCH) --export=All,Filename=$$(RESULT_DIR)/$$(GIT_TREE_HASH)/grid_vi_$(4)_$(1)_$(2)_$(3)_$(5).o --output=$$@ --error=$$(RESULT_DIR)/$$(GIT_TREE_HASH)/parse_grid_vi_$(4)_$(1)_$(2)_$(3)_$(5).e --job-name=parse_grid_vi_$(4) --dependency=$$(JOBID) slurm/parse.sh
	$$(SLEEP)
endif

.PRECIOUS: $$(RESULT_DIR)/$$(GIT_TREE_HASH)/grid_vi_$(4)_$(1)_$(2)_$(3)_$(5).o
endef
$(foreach domain,$(DOMAINS),$(foreach assignment_id,$($(domain)_ids),$(foreach horizon,$($(domain)_horizons),$(foreach n_bin,$(NBinD),$(foreach rep,$(REPETITION),$(eval $(call f_grid_vi,$(assignment_id),$(n_bin),$(horizon),$(domain),$(rep))))))))

define f_grid_vi_jsons =
grid_vi_$(1)_jsons = $$(foreach instance,$$($(1)_ids),$$(foreach n_bin,$$(NBinD),$$(foreach horizon,$$($(1)_horizons),$$(foreach rep,$$(REPETITION),$$(RESULT_DIR)/$$(GIT_TREE_HASH)/grid_vi_$(1)_$$(instance)_$$(n_bin)_$$(horizon)_$$(rep).json))))
endef

$(foreach domain,$(DOMAINS),$(eval $(call f_grid_vi_jsons,$(domain))))

define f_domain =
$(1)_jsons = $$(foreach algo,$$(ALGORITHMS),$$($$(algo)_$(1)_jsons))

$(1)_pngs = $$(foreach assignment_id,$$($(1)_ids),figs/$$(GIT_TREE_HASH)/$(1)_online_$$(assignment_id).png) 
# $$(foreach assignment_id,$$($(1)_ids),$$(foreach horizon,$$($(1)_horizons),figs/$$(GIT_TREE_HASH)/$(1)_offline_$$(assignment_id)_$$(horizon).png))

$(1)_dependencies = $$(subst $$(SPACE),$$(COMMA),$$(foreach algo,$$(ALGORITHMS),$$(algo)_$(1) parse_$$(algo)_$(1)))

$$(RESULT_DIR)/$$(GIT_TREE_HASH)/$(1)_jsons.txt: $$($(1)_jsons)

$$(RESULT_DIR)/$$(GIT_TREE_HASH)/$(1).json: scripts/collect_from_file.py $$($(1)_jsons) $$(RESULT_DIR)/$$(GIT_TREE_HASH)/$(1)_jsons.txt
	$$(eval JOBIDS=$$(shell squeue -u $$(USER) -n $$($(1)_dependencies) -o %A | tail -n +2 | paste -sd "," -))
	$$(info $$(JOBIDS))
	$$(SBATCH) --export=All,Filenames=$$(RESULT_DIR)/$$(GIT_TREE_HASH)/$(1)_jsons.txt --output=$$@ --error=$$(RESULT_DIR)/$$(GIT_TREE_HASH)/collect_$(1).e --job-name=collect_$(1) --dependency=$$(JOBIDS) slurm/collect.sh

html/$$(GIT_TREE_HASH)/report_$(1).html: $$(RESULT_DIR)/$$(GIT_TREE_HASH)/$(1).json scripts/report.py html/$$(GIT_TREE_HASH)
	$$(eval JOBIDS=$$(shell squeue -u $$(USER) -n $$($(1)_dependencies),collect_$(1) -o %A | tail -n +2 | paste -sd "," -))
	$$(info $$(JOBIDS))
	$$(eval Filenames="$$(RESULT_DIR)/$$(GIT_TREE_HASH)/$(1).json")
	$$(SBATCH) --export=All,Filenames=$$(Filenames),Domain=$(1),Horizons="$$($(1)_horizons)",NTrials="$$(NTRIAL)",NBins="$$(NBinD)",Ids="$$($(1)_ids)",Output="html/$$(GIT_TREE_HASH)/report_$(1).html" --output=$$@ --error=html/$$(GIT_TREE_HASH)/report_$(1)_html.e --job-name=$(1)_html --dependency=$$(JOBIDS) slurm/report.sh

$(1)_html = html/$$(GIT_TREE_HASH)/report_$(1).html
endef


$(foreach domain,$(DOMAINS),$(eval $(call f_domain,$(domain))))

figs/$(GIT_TREE_HASH):
	mkdir -p figs/$(GIT_TREE_HASH)

define f_png =
figs/$$(GIT_TREE_HASH)/$(1)_online_$(2).png: $$(RESULT_DIR)/$$(GIT_TREE_HASH)/$(1).json $$(SCRIPT_DIR)/plot.py | figs/$$(GIT_TREE_HASH)
	$$(eval JOBIDS=$$(shell squeue -u $$(USER) -n $$($(1)_dependencies),collect_$(1) -o %A | tail -n +2 | paste -sd "," -))
	$$(info $$(JOBIDS))
	$$(SBATCH) --export=All,Filename=$$(RESULT_DIR)/$$(GIT_TREE_HASH)/$(1).json,Domain=$(1),AssignmentId=$(2),Horizon=$(3) --output=figs/$$(GIT_TREE_HASH)/plot_$(1)_online_$(2)_$(3).o --error=figs/$$(GIT_TREE_HASH)/plot_$(1)_online_$(2)_$(3).e --job-name=plot --dependency=$$(JOBIDS) slurm/plot.sh
endef

$(foreach domain,$(DOMAINS),$(foreach assignment_id,$($(domain)_ids),$(foreach horizon,$($(domain)_horizons),$(eval $(call f_png,$(domain),$(assignment_id),$(horizon))))))

pngs = $(baker_pngs) $(baker5_com_pngs) $(spelling_pngs) $(baker5_pngs) 


.PHONY: uai_pngs
uai_pngs: figs/$(GIT_TREE_HASH)/reset5_online_303.png figs/$(GIT_TREE_HASH)/reset5_online_302.png figs/$(GIT_TREE_HASH)/spelling_online_3.png figs/$(GIT_TREE_HASH)/spelling_online_8.png figs/$(GIT_TREE_HASH)/blocks_online_1.png figs/$(GIT_TREE_HASH)/blocks_online_2.png

.PHONY: uai_html
uai_html: html/$(GIT_TREE_HASH)/report_spelling.html html/$(GIT_TREE_HASH)/report_reset5.html html/$(GIT_TREE_HASH)/report_blocks.html 

print-%  : ; @echo $* = $($*)

variable-%  : $($*)
	@echo $($*)
	$(MAKE) $($*)

clean-%  : $($*)
	@echo $($*)
	rm $($*)
