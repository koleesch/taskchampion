////////////////////////////////////////////////////////////////////////////////
// taskwarrior - a command line task list manager.
//
// Copyright 2006 - 2011, Paul Beckingham, Federico Hernandez.
// All rights reserved.
//
// This program is free software; you can redistribute it and/or modify it under
// the terms of the GNU General Public License as published by the Free Software
// Foundation; either version 2 of the License, or (at your option) any later
// version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE.  See the GNU General Public License for more
// details.
//
// You should have received a copy of the GNU General Public License along with
// this program; if not, write to the
//
//     Free Software Foundation, Inc.,
//     51 Franklin Street, Fifth Floor,
//     Boston, MA
//     02110-1301
//     USA
//
////////////////////////////////////////////////////////////////////////////////

#include <fstream>
#include <sstream>
#include <Context.h>
#include <URI.h>
#include <Transport.h>
#include <text.h>
#include <CmdPush.h>

extern Context context;

////////////////////////////////////////////////////////////////////////////////
CmdPush::CmdPush ()
{
  _keyword     = "push";
  _usage       = "task push URL";
  _description = "Pushes the local *.data files to the URL.";
  _read_only   = true;
  _displays_id = false;
}

////////////////////////////////////////////////////////////////////////////////
// Transfers the local data (from rc.location.data) to the remote path.  Because
// this is potentially on another machine, no checking can be performed.
int CmdPush::execute (const std::string&, std::string& output)
{
  std::string file = trim (context.task.get ("description"));

  Uri uri (file, "push");
  uri.parse ();

  if (uri.data.length ())
  {
		Directory location (context.config.get ("data.location"));

		Transport* transport;
		if ((transport = Transport::getTransport (uri)) != NULL )
		{
			transport->send (location.data + "/{pending,undo,completed}.data");
			delete transport;
		}
		else
		{
      // Verify that files are not being copied from rc.data.location to the
      // same place.
      if (Directory (uri.path) == Directory (context.config.get ("data.location")))
        throw std::string ("Cannot push files when the source and destination are the same.");

      // copy files locally
      if (! Path (uri.data).is_directory ())
        throw std::string ("The uri '") + uri.path + "' is not a local directory.";

      std::ifstream ifile1 ((location.data + "/undo.data").c_str(), std::ios_base::binary);
      std::ofstream ofile1 ((uri.path      + "/undo.data").c_str(), std::ios_base::binary);
      ofile1 << ifile1.rdbuf();

      std::ifstream ifile2 ((location.data + "/pending.data").c_str(), std::ios_base::binary);
      std::ofstream ofile2 ((uri.path      + "/pending.data").c_str(), std::ios_base::binary);
      ofile2 << ifile2.rdbuf();

      std::ifstream ifile3 ((location.data + "/completed.data").c_str(), std::ios_base::binary);
      std::ofstream ofile3 ((uri.path      + "/completed.data").c_str(), std::ios_base::binary);
      ofile3 << ifile3.rdbuf();
		}

    output += "Local tasks transferred to " + uri.data + "\n";
  }
  else
    throw std::string ("No uri was specified for the push.  Either specify "
                       "the uri of a remote .task directory, or create a "
                       "'push.default.uri' entry in your .taskrc file.");

  return 0;
}

////////////////////////////////////////////////////////////////////////////////
